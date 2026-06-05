# ternary-diehard

**Three-state cellular automata: when Dead/Idle/Alive produces richer dynamics than Dead/Alive.**

Conway's Game of Life uses two states — dead and alive. This crate explores what happens when you add a third **idle** (refractory) state. The result: spiral waves, traveling pulses, self-repairing patterns, and dynamics that binary Life simply cannot produce.

---

## Why Three States?

Binary cellular automata have a problem: patterns either die out or grow forever. The third state (Idle/Refractory) acts as a **recovery period** — a cell that just became alive can't immediately become alive again. This is exactly how neurons work:

```
Dead → (neighbors activate) → Alive → (cooldown) → Idle → (rest) → Dead
```

This three-state cycle is the **Greenberg-Hastings model** from mathematical biology, which produces spiral waves identical to those in cardiac tissue.

---

## Rule Variants

| Variant | Dead→Alive | Alive→Idle | Idle→Dead |
|---------|-----------|-----------|----------|
| ThreeStateLife | ≥2 alive neighbors | always | always |
| HighLifeTernary | 3 or 6 neighbors | always | always |
| DayAndNightTernary | 4,6,7,8 neighbors | always | always |

---

## Architecture

- **`TritCell`** — `Dead(-1)`, `Idle(0)`, `Alive(+1)`
- **`LifeGrid`** — Generic ternary grid with step/count/query operations
- **`ThreeStateLife`** — Standard 3-state rules
- **`HighLifeTernary`** — HighLife variant with B6 birth rule
- **`DayAndNightTernary`** — Day & Night symmetry variant
- **`detect_oscillation()`** — Find period of population oscillation
- **`find_still_life()`** — Detect static (unchanging) patterns
- **`PopulationStats`** — Track alive/idle/dead counts over time

---

## Quick Start

```rust
use ternary_diehard::{LifeGrid, ThreeStateLife, TritCell, PopulationStats};

let mut grid = LifeGrid::random(30, 30, 42);
let rules = ThreeStateLife::new();
let mut stats = PopulationStats::new();

for _ in 0..100 {
    grid.step(&rules);
    stats.record(&grid);
}

println!("Final population: {:?}", stats.last());
println!("Oscillation period: {:?}", detect_oscillation(&stats.history));
```

---

## Ecosystem

- **ternary-spiral** — RPS cyclic dominance spirals (related spatial dynamics)
- **ternary-grid** — Grid utilities
- **ternary-step** — Stepping rules for ternary automata
- **ternary-morph** — Morphological operations on ternary grids

## License

MIT
