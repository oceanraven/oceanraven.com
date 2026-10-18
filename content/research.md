I'm interested in **mechanistic interpretability**: understanding neural networks
by reverse-engineering the algorithms they learn, rather than treating them as
black boxes. If a model can do something, I want to know *how* it does it, down
to the level of individual features and circuits.

## Current interests

- **Features and superposition.** Models represent far more concepts than they
  have neurons. I'm interested in how sparse autoencoders and related dictionary
  learning methods recover these features, and where they fall short.
- **Circuits.** Tracing how features compose across layers to implement specific
  behaviors, from simple pattern completion to multi-step reasoning.
- **Tooling for interpretability.** Good research needs good tools. I enjoy
  building fast, reliable infrastructure for running and visualizing
  interpretability experiments, often in Rust.

## Why this matters

As models become more capable, being able to look inside them and verify what
they are doing becomes more important. I think interpretability is one of the
most promising paths toward models we can actually trust.