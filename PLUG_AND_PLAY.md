# PLUG_AND_PLAY — Diehard

> Ternary cellular automata — Life-like rules on trit grids

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
ternary-diehard = { git = "https://github.com/SuperInstance/ternary-diehard" }
```

Use in your code:

```rust
use ternary_diehard::{HighLifeTernary, Simulator};

let mut sim = HighLifeTernary::new(64, 64);
sim.randomize();
for _ in 0..100 { sim.step(); }
```

## 📚 Available Documentation

| Document | Description |
|----------|-------------|
| `docs/FROM_BINARY.md` | Understanding ternary concepts as a binary programmer |
| `docs/MIGRATION.md` | Version migration guide |
| `docs/FUTURE-INTEGRATION.md` | Planned features and roadmap |

## 🔗 Integration

This crate is part of the [SuperInstance ternary fleet](https://github.com/SuperInstance). It uses the canonical `Ternary` type from `ternary-types` for cross-crate compatibility.

## 📄 License

MIT
