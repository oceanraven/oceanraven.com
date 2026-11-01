import argparse
import json
import time
import warnings
from pathlib import Path
from typing import TypedDict, cast

import requests
import torch
from sae_lens import SAE
from transformer_lens.model_bridge import TransformerBridge

TEXT = (
    "I'm an undergraduate at POSTECH working on mechanistic interpretability. "
    "I try to reverse-engineer how neural networks represent concepts and carry out computations, "
    "one circuit at a time. When I'm not staring at activations, I build things in Rust, "
    "including this site."
)
DEVICE = "cuda" if torch.cuda.is_available() else "xpu" if torch.xpu.is_available() else "cpu"
MODEL = "gpt2"
RELEASE = "gpt2-small-res-jb"
LAYER = 8
NUM_CANDIDATES = 30
CANDIDATE_POOL = 60
CHOSEN: list[int] = [8432, 13659, 18953, 7788, 21663, 15870, 23745]
MIN_ACTIVATION = 0.05
REQUEST_DELAY = 0.1

HERE = Path(__file__).resolve().parent
CACHE_PATH = HERE / "neuronpedia_cache.json"
OUTPUT_PATH = HERE.parents[1] / "content" / "features.json"

warnings.filterwarnings(
    "ignore",
    message=r"\nThis SAE has non-empty model_from_pretrained_kwargs",
    category=UserWarning,
)


class FeatureInfo(TypedDict):
    label: str
    max_act: float


Cache = dict[str, FeatureInfo]


def load() -> tuple[TransformerBridge, SAE]:
    loaded = SAE.from_pretrained(
        release=RELEASE,
        sae_id=f"blocks.{LAYER}.hook_resid_pre",
        device=DEVICE,
    )
    sae = cast(SAE, loaded[0] if isinstance(loaded, tuple) else loaded)

    model = TransformerBridge.boot_transformers(MODEL, device=DEVICE)
    kwargs: dict = sae.cfg.metadata.model_from_pretrained_kwargs or {}
    model.enable_compatibility_mode(**kwargs)

    return model, sae


def tokenize(model: TransformerBridge) -> tuple[torch.Tensor, list[str]]:
    tokens = model.to_tokens(TEXT)
    strs = cast(list[str], model.to_str_tokens(TEXT))
    return tokens, strs


def feature_acts(model: TransformerBridge, sae: SAE, tokens: torch.Tensor) -> torch.Tensor:
    hook: str = sae.cfg.metadata.hook_name
    with torch.no_grad():
        _, cache = model.run_with_cache(tokens, names_filter=hook)
        acts = sae.encode(cache[hook])
    return acts[0, 1:].cpu()


def prepare() -> tuple[torch.Tensor, list[str], str]:
    model, sae = load()
    tokens, strs = tokenize(model)
    acts = feature_acts(model, sae, tokens)

    neuronpedia_id: str | None = sae.cfg.metadata.neuronpedia_id
    if neuronpedia_id is None:
        raise RuntimeError(f"{RELEASE} layer {LAYER} has no Neuronpedia id")

    return acts, strs[1:], neuronpedia_id


def load_cache() -> Cache:
    if CACHE_PATH.exists():
        return json.loads(CACHE_PATH.read_text())
    return {}


def save_cache(cache: Cache) -> None:
    CACHE_PATH.write_text(json.dumps(cache, indent=2, ensure_ascii=False))


def feature_info(neuronpedia_id: str, feature: int, cache: Cache) -> FeatureInfo:
    key = str(feature)
    if key not in cache:
        url = f"https://www.neuronpedia.org/api/feature/{neuronpedia_id}/{feature}"
        response = requests.get(url, timeout=10)
        response.raise_for_status()
        data = response.json()

        explanations = data.get("explanations") or []
        label = explanations[0]["description"].strip() if explanations else "(no explanation)"
        cache[key] = {"label": label, "max_act": float(data["maxActApprox"])}
        save_cache(cache)
        time.sleep(REQUEST_DELAY)
    return cache[key]


def explore() -> None:
    acts, words, neuronpedia_id = prepare()

    values: torch.Tensor  # [d_sae]
    positions: torch.Tensor  # [d_sae]
    values, positions = acts.max(dim=0)
    pool = values.topk(CANDIDATE_POOL)

    cache = load_cache()
    rows: list[tuple[float, int, int, float, float, str, str]] = []
    for value, feature in zip(pool.values.tolist(), pool.indices.tolist()):
        info = feature_info(neuronpedia_id, feature, cache)
        if info["max_act"] <= 0:
            continue
        ratio = value / info["max_act"]
        lit = int((acts[:, feature] / info["max_act"] >= MIN_ACTIVATION).sum().item())
        word = words[int(positions[feature].item())]
        rows.append((ratio, feature, lit, value, info["max_act"], word, info["label"]))

    rows.sort(key=lambda row: row[0], reverse=True)

    print(f"{'feature':>7}  {'ratio':>5}  {'lit':>3}  {'here':>6}  {'max':>6}  {'token':<16}  label")
    for ratio, feature, lit, value, max_act, word, label in rows[:NUM_CANDIDATES]:
        print(
            f"{feature:>7}  {ratio:>5.2f}  {lit:>3}  {value:>6.2f}  {max_act:>6.2f}  "
            f"{word!r:<16}  {label}"
        )


def sparse_activations(column: torch.Tensor, max_act: float) -> list[list[float]]:
    normalized: list[float] = (column / max_act).clamp(max=1.0).tolist()
    return [
        [position, round(value, 2)]
        for position, value in enumerate(normalized)
        if value >= MIN_ACTIVATION
    ]


def preview(words: list[str], activations: list[list[float]]) -> str:
    by_position = {int(position): value for position, value in activations}
    parts: list[str] = []
    for position, word in enumerate(words):
        value = by_position.get(position)
        if value is None:
            parts.append(word)
        else:
            leading = word[: len(word) - len(word.lstrip())]
            parts.append(f"{leading}[{word.lstrip()}:{value:.2f}]")
    return "".join(parts)


def export() -> None:
    if not CHOSEN:
        print("CHOSEN is empty. Run `explore` first and add feature ids to CHOSEN.")
        return

    acts, words, neuronpedia_id = prepare()
    cache = load_cache()

    features: list[dict[str, object]] = []
    for feature in CHOSEN:
        info = feature_info(neuronpedia_id, feature, cache)
        activations = sparse_activations(acts[:, feature], info["max_act"])
        if not activations:
            print(f"warning: feature {feature} has no activation above {MIN_ACTIVATION}")

        print(f"\n#{feature} · {info['label']}")
        print(preview(words, activations))

        features.append({"id": feature, "label": info["label"], "activations": activations})

    result = {
        "source": {"model": "gpt2-small", "sae": "res-jb", "layer": LAYER, "neuronpedia": neuronpedia_id},
        "tokens": words,
        "features": features,
    }
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n")
    print(f"\nwrote {OUTPUT_PATH}")


def main() -> None:
    parser = argparse.ArgumentParser(description="Extract GPT-2 SAE features for the home page.")
    parser.add_argument("mode", choices=["explore", "export"])
    args = parser.parse_args()

    if args.mode == "explore":
        explore()
    else:
        export()


if __name__ == "__main__":
    main()
