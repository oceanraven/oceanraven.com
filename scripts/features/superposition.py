import json
import math
from pathlib import Path

import torch
from tqdm import tqdm

DEVICE = "cuda" if torch.cuda.is_available() else "xpu" if torch.xpu.is_available() else "cpu"
N_FEATURES = 5
N_HIDDEN = 2
N_SPARSITIES = 11
MIN_DENSITY = 0.01
SPARSITIES = [round(1 - MIN_DENSITY ** (i / (N_SPARSITIES - 1)), 3) for i in range(N_SPARSITIES)]
N_SEEDS = 64
STEPS = 10_000
BATCH = 1024
LR = 1e-3
SEED = 0
EVAL_SIZE = 100_000
EVAL_SEED = 1234

IMPORTANCE = torch.tensor([0.9 ** i for i in range(N_FEATURES)], device=DEVICE)

HERE = Path(__file__).resolve().parent
OUTPUT_PATH = HERE.parents[1] / "content" / "superposition.json"


def sample_batch(
        density: torch.Tensor, size: int, generator: torch.Generator | None = None
) -> torch.Tensor:
    shape = (*density.shape, size, N_FEATURES)
    values = torch.rand(shape, device=DEVICE, generator=generator)
    keep = torch.rand(shape, device=DEVICE, generator=generator) < density[..., None, None]
    return values * keep


def reconstruction_loss(x: torch.Tensor, W: torch.Tensor, b: torch.Tensor) -> torch.Tensor:
    out = torch.relu(x @ W.transpose(-1, -2) @ W + b.unsqueeze(-2))
    return (IMPORTANCE * (x - out) ** 2).sum(dim=-1).mean(dim=-1)


def train() -> tuple[torch.Tensor, torch.Tensor]:
    torch.manual_seed(SEED)

    n_models = N_SPARSITIES * N_SEEDS
    density = (1 - torch.tensor(SPARSITIES, device=DEVICE)).repeat_interleave(N_SEEDS)

    scale = math.sqrt(2 / (N_HIDDEN + N_FEATURES))
    W = torch.nn.Parameter(torch.randn(n_models, N_HIDDEN, N_FEATURES, device=DEVICE) * scale)
    b = torch.nn.Parameter(torch.zeros(n_models, N_FEATURES, device=DEVICE))

    optimizer = torch.optim.Adam([W, b], lr=LR)

    for _ in tqdm(range(STEPS), desc="training", leave=False):
        loss = reconstruction_loss(sample_batch(density, BATCH), W, b).sum()

        optimizer.zero_grad()
        loss.backward()
        optimizer.step()

    return (
        W.detach().view(N_SPARSITIES, N_SEEDS, N_HIDDEN, N_FEATURES),
        b.detach().view(N_SPARSITIES, N_SEEDS, N_FEATURES),
    )


def evaluate(W: torch.Tensor, b: torch.Tensor, sparsity: float) -> torch.Tensor:
    generator = torch.Generator(device=DEVICE).manual_seed(EVAL_SEED)
    density = torch.tensor(1 - sparsity, device=DEVICE)
    with torch.no_grad():
        x = sample_batch(density, EVAL_SIZE, generator)
        return reconstruction_loss(x, W, b)


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


def describe(sparsity: float, loss: float, W: torch.Tensor) -> None:
    norms = W.norm(dim=0).tolist()
    angles = [
        f"{math.degrees(math.atan2(y, x)):4.0f}" if norm >= 0.1 else "   -"
        for (x, y), norm in zip(W.T.tolist(), norms)
    ]
    print(
        f"S={sparsity:.3f}  loss: {loss:.5f}  "
        f"norms: {' '.join(f'{n:.2f}' for n in norms)}  angles: {' '.join(angles)}"
    )


def main():
    weights, biases = train()

    steps = []
    for sparsity, W_seeds, b_seeds in zip(SPARSITIES, weights, biases):
        losses = evaluate(W_seeds, b_seeds, sparsity)
        best = int(losses.argmin().item())
        W = align(W_seeds[best].cpu())

        describe(sparsity, losses[best].item(), W)
        vectors = [[round(x, 3), round(y, 3)] for x, y in W.T.tolist()]
        steps.append({"sparsity": sparsity, "vectors": vectors})

    result = {
        "importance": [round(v, 3) for v in IMPORTANCE.tolist()],
        "steps": steps,
    }
    OUTPUT_PATH.write_text(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()