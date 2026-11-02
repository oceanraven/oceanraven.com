import json
from pathlib import Path
import math

import torch
from tqdm import tqdm

DEVICE = "cuda" if torch.cuda.is_available() else "xpu" if torch.xpu.is_available() else "cpu"
N_FEATURES = 5
N_HIDDEN = 2
SPARSITIES = [0.0, 0.5, 0.7, 0.8, 0.9, 0.95, 0.99]
STEPS = 10_000
BATCH = 1024
LR = 1e-3
N_SEEDS = 64
SEED = 0
EVAL_SIZE = 100_000
EVAL_SEED = 1234
LOSS_TOLERANCE = 0.02
REPRESENTATION_TOLERANCE = 0.5

IMPORTANCE = torch.tensor([0.9 ** i for i in range(N_FEATURES)], device=DEVICE)

HERE = Path(__file__).resolve().parent
OUTPUT_PATH = HERE.parents[1] / "content" / "superposition.json"


def sample_batch(
        sparsity: float, size: int = BATCH, generator: torch.Generator | None = None
) -> torch.Tensor:
    values = torch.rand(size, N_FEATURES, device=DEVICE, generator=generator)
    keep = torch.rand(size, N_FEATURES, device=DEVICE, generator=generator) < (1 - sparsity)
    return values * keep


def reconstruction_loss(x: torch.Tensor, W: torch.Tensor, b: torch.Tensor) -> torch.Tensor:
    out = torch.relu(x @ W.transpose(-1, -2) @ W + b.unsqueeze(-2))
    return (IMPORTANCE * (x - out) ** 2).sum(dim=-1).mean(dim=-1)


def represented(W: torch.Tensor) -> int:
    return int((W.norm(dim=0) > REPRESENTATION_TOLERANCE).sum().item())


def evaluate(W: torch.Tensor, b: torch.Tensor, sparsity: float) -> list[float]:
    generator = torch.Generator(device=DEVICE).manual_seed(EVAL_SEED)
    with torch.no_grad():
        x = sample_batch(sparsity, EVAL_SIZE, generator)
        return reconstruction_loss(x, W, b).tolist()


def train(sparsity: float) -> list[tuple[torch.Tensor, float]]:
    torch.manual_seed(SEED)

    scale = math.sqrt(2 / (N_HIDDEN + N_FEATURES))
    W = torch.nn.Parameter(torch.randn(N_SEEDS, N_HIDDEN, N_FEATURES, device=DEVICE) * scale)
    b = torch.nn.Parameter(torch.zeros(N_SEEDS, N_FEATURES, device=DEVICE))

    optimizer = torch.optim.Adam([W, b], lr=LR)

    for _ in tqdm(range(STEPS), desc=f"S={sparsity:.2f}", leave=False):
        x = sample_batch(sparsity, N_SEEDS * BATCH).view(N_SEEDS, BATCH, N_FEATURES)
        loss = reconstruction_loss(x, W, b).sum()

        optimizer.zero_grad()
        loss.backward()
        optimizer.step()

    W, b = W.detach(), b.detach()
    return list(zip(W.cpu().unbind(0), evaluate(W, b, sparsity)))


def align(W: torch.Tensor) -> torch.Tensor:
    x0, y0 = W[0, 0].item(), W[1, 0].item()
    if math.hypot(x0, y0) < 1e-3:
        return W
    theta = math.atan2(y0, x0)
    phi = math.pi / 2 - theta
    R = torch.tensor([[math.cos(phi), -math.sin(phi)],
                      [math.sin(phi), math.cos(phi)]])
    W = R @ W

    for i in range(1, N_FEATURES):
        if abs(W[0, i]) > 0.05:
            if W[0, i] < 0:
                W[0, :] *= -1
            break

    return W


def describe(sparsity: float, W: torch.Tensor) -> None:
    norms = W.norm(dim=0).tolist()
    angles = [
        f"{math.degrees(math.atan2(y, x)):4.0f}" if norm >= 0.1 else "   -"
        for (x, y), norm in zip(W.T.tolist(), norms)
    ]
    print(f"S={sparsity:.2f}  norms: {' '.join(f'{n:.2f}' for n in norms)}  angles: {' '.join(angles)}")


def mirror(W: torch.Tensor) -> torch.Tensor:
    M = W.clone()
    M[0, :] *= -1
    return M


def distance(A: torch.Tensor, B: torch.Tensor) -> float:
    return ((A - B) ** 2).sum().item()


def main():
    steps = []
    previous: torch.Tensor | None = None
    for sparsity in SPARSITIES:
        results = train(sparsity)
        best_W, best_loss = min(results, key=lambda result: result[1])
        candidates = [
            align(W)
            for W, loss in results
            if loss <= best_loss * (1 + LOSS_TOLERANCE) and represented(W) == represented(best_W)
        ]
        print(f"S={sparsity:.2f}  best loss: {best_loss:.5f}  candidates: {len(candidates)}")

        if previous is None:
            W = align(best_W)
        else:
            options = candidates + [mirror(W) for W in candidates]
            W = min(options, key=lambda option: distance(option, previous))
        previous = W

        describe(sparsity, W)
        vectors = [[round(x, 3), round(y, 3)] for x, y in W.T.tolist()]
        steps.append({"sparsity": sparsity, "vectors": vectors})

    result = {
        "importance": [round(v, 3) for v in IMPORTANCE.tolist()],
        "steps": steps,
    }
    OUTPUT_PATH.write_text(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
