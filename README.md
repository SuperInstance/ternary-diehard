# ternary-diehard

> Three-state cellular automata: Conway's Game of Life extended with a quiescent "Idle" state.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## What problem does this solve?

Conway's Game of Life is a binary automaton: cells are **Alive** or **Dead**. In real biological tissues, however, cells often enter a **quiescent** or **refractory** state—neither fully active nor apoptotic. Think of cell-cycle arrest, dormant stem-cell niches, or neurons in relative hyperpolarization. A ternary automaton with states `Dead (-1)`, `Idle (0)`, and `Alive (1)` lets us model these intermediate physiological conditions within a discrete spatial framework.

Mathematically, this is a **totalistic cellular automaton** on a 2-D square lattice with Moore neighborhood (8 neighbors). The update function

```
s'(x, y) = f( s(x, y),  N_alive(x, y),  N_active(x, y) )
```

depends on the cell's current state and the counts of Alive and active (Alive+Idle) neighbors. This crate implements three well-known rule families—**ThreeStateLife**, **HighLifeTernary**, and **DayAndNightTernary**—adapted to include an Idle refractory state.

---

## The science

### Trit encoding of cell fate

| Trit | State | Biological analogue |
|------|-------|---------------------|
| `-1` | Dead  | Apoptotic / empty lattice site |
| ` 0` | Idle  | Quiescent / refractory / G₀-arrested |
| ` 1` | Alive | Mitotically active / firing |

### ThreeStateLife rules

A direct extension of Conway's Life:

- **Birth**: Dead → Alive if exactly **3** Alive neighbors.
- **Survival**: Alive persists if **2 or 3** Alive neighbors; dies otherwise.
- **Refractory transition**: Alive → **Idle** if exactly **2** Alive neighbors (a graceful degradation rather than immediate death).
- **Idle fate**: Idle → Alive if **3** Alive neighbors; otherwise → Dead.

The Idle state acts as a one-generation memory of recent activity, preventing certain high-frequency oscillations and enriching the space of still-life configurations.

### HighLifeTernary rules

HighLife (B36/S23) is famous for supporting a replicator. In the ternary variant:

- Birth: Dead → Alive on **3 or 6** Alive neighbors.
- Survival: Alive persists on **2 or 3**; else → Idle (not directly Dead).
- Idle always decays to Dead in one step.

This produces longer-lived debris and different glide-symmetry dynamics compared to standard Life.

### DayAndNightTernary rules

Day & Night (B3678/S34678) is self-complementary: swapping Alive ↔ Dead preserves the rule. Our ternary version uses:

- Birth on **{3, 6, 7, 8}** Alive neighbors for Dead cells; active-neighbor count for Idle cells.
- Survival on **{3, 4, 6, 7, 8}** Alive neighbors; else → Idle.

Because Day & Night is structurally rich in still lifes and high-density oscillators, the Idle state here captures transient boundaries between day-like (high density) and night-like (low density) regions.

### Analysis utilities

- **`detect_oscillation(history)`** — Scans a population time-series for periodicity (periods 2–10), useful for identifying limit cycles in small toroidal grids.
- **`PopulationStats`** — Computes min, max, mean, and variance; `is_stable()` flags near-constant populations.
- **`find_still_life(grid)`** — Tests whether a configuration is a fixed point under ThreeStateLife.

---

## Architecture

```text
┌────────────────────────────────────────────┐
│            LifeGrid                        │
│  width × height  |  toroidal Moore nbhd    │
│  ├── alive_neighbors(x, y)  -> usize       │
│  ├── active_neighbors(x, y) -> usize       │
│  ├── population()           -> Alive count │
│  └── active_count()         -> Alive+Idle  │
└─────────────┬──────────────────────────────┘
              │
    ┌─────────┼─────────┐
    ▼         ▼         ▼
┌────────┐ ┌──────────┐ ┌──────────────┐
│ThreeStateLife│ │HighLifeTernary│ │DayAndNightTernary│
│ step() │ │ step()   │ │ step()       │
│ B3/S23 │ │ B36/S23  │ │ B3678/S34678 │
│ +Idle  │ │ +Idle    │ │ +Idle        │
└────────┘ └──────────┘ └──────────────┘

Utilities:
  detect_oscillation()  ──► Option<period>
  PopulationStats::compute()  ──► mean, variance, is_stable()
  find_still_life()     ──► bool
```

---

## Getting Started

Add to `Cargo.toml`:

```toml
[dependencies]
ternary-diehard = { git = "https://github.com/SuperInstance/ternary-diehard.git" }
```

Run a classic blinker under ThreeStateLife:

```rust
use ternary_diehard::{ThreeStateLife, TritCell};

fn main() {
    let mut sim = ThreeStateLife::new(10, 10);

    // Vertical blinker at the centre
    sim.set_pattern(&[(4, 3), (4, 4), (4, 5)]);

    for gen in 0..5 {
        println!("gen {} | population = {}", gen, sim.grid.population());
        sim.step();
    }
}
```

Compile and run:

```bash
cargo run
```

---

## Running the Tests

```bash
cargo test
```

The 12 tests verify state logic, rule tables, and population analytics:

| Test | What it verifies |
|------|------------------|
| `trit_cell_active` | `is_active()` correctly distinguishes Dead from Idle/Alive. |
| `three_state_life_empty` | An empty grid (all Dead) remains empty after one step. |
| `three_state_life_birth` | A Dead cell with exactly 3 Alive Moore neighbors becomes Alive. |
| `three_state_life_death_overcrowded` | An Alive cell with 5 Alive neighbors dies (overcrowding rule). |
| `three_state_life_survives` | A 2×2 block is a stable square: each cell has 3 Alive neighbors and survives. |
| `highlife_born_on_six` | HighLife birth on 6 neighbors: a Dead cell with 6 Alive neighbors becomes Alive. |
| `highlife_idle_transition` | HighLife degradation: a lone Alive cell (0 neighbors) transitions to Idle, not directly Dead. |
| `day_and_night_birth` | Day & Night birth on 3 neighbors works correctly from a Dead state. |
| `detect_oscillation_period2` | A period-2 population sequence `[10,5,10,5,10,5]` is detected as oscillating with period 2. |
| `detect_oscillation_none` | Monotonic growth yields `None`—no periodicity is falsely reported. |
| `population_stats` | `PopulationStats` accurately computes min, max, and arithmetic mean. |
| `find_still_life_empty` | The all-Dead grid is confirmed as a still life under ThreeStateLife. |

---

## Related crates in the ternary ecosystem

- [`ternary-automata`](https://github.com/SuperInstance/ternary-automata) — General-purpose totalistic and outer-totalistic CA engine with pluggable rules.
- [`ternary-life`](https://github.com/SuperInstance/ternary-life) — Classic 2-state Life implementations and pattern libraries.
- [`ternary-cell`](https://github.com/SuperInstance/ternary-cell) — Single-cell and small-colony stochastic simulators for lineage tracing.
- [`ternary-sandpile`](https://github.com/SuperInstance/ternary-sandpile) — Abelian sandpile dynamics on ternary height fields.

---

## License

This project is licensed under the [MIT License](LICENSE).
