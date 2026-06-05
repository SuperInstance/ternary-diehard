#![allow(dead_code)]

// ternary-diehard: Game of Life variants on ternary grids.
// Each cell is Dead (-1), Idle (0), or Alive (1).

/// A single ternary cell state.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TritCell {
    Dead = -1,
    Idle = 0,
    Alive = 1,
}

impl TritCell {
    /// Returns true if the cell is Idle or Alive (i.e. not Dead).
    pub fn is_active(&self) -> bool {
        matches!(self, TritCell::Idle | TritCell::Alive)
    }

    /// Construct from a trit value: -1 => Dead, 0 => Idle, 1 => Alive.
    pub fn from_trit(t: i8) -> Option<TritCell> {
        match t {
            -1 => Some(TritCell::Dead),
            0 => Some(TritCell::Idle),
            1 => Some(TritCell::Alive),
            _ => None,
        }
    }

    /// Convert to its trit value.
    pub fn to_trit(&self) -> i8 {
        match self {
            TritCell::Dead => -1,
            TritCell::Idle => 0,
            TritCell::Alive => 1,
        }
    }
}

/// A 2D grid of ternary cells with toroidal (wrapping) topology.
pub struct LifeGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<TritCell>>,
    pub generation: u64,
}

impl LifeGrid {
    /// Create a new grid, all cells Dead.
    pub fn new(width: usize, height: usize) -> Self {
        LifeGrid {
            width,
            height,
            cells: vec![vec![TritCell::Dead; width]; height],
            generation: 0,
        }
    }

    /// Set cell at (x, y) to the given state.
    pub fn set(&mut self, x: usize, y: usize, cell: TritCell) {
        self.cells[y][x] = cell;
    }

    /// Get cell at (x, y).
    pub fn get(&self, x: usize, y: usize) -> TritCell {
        self.cells[y][x]
    }

    /// Count Alive neighbors in the Moore neighborhood (8 cells), with wrapping.
    pub fn alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in [-1i32, 0, 1] {
            for dx in [-1i32, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = ((x as i32 + dx).rem_euclid(self.width as i32)) as usize;
                let ny = ((y as i32 + dy).rem_euclid(self.height as i32)) as usize;
                if self.cells[ny][nx] == TritCell::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    /// Count Idle + Alive neighbors in the Moore neighborhood, with wrapping.
    pub fn active_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in [-1i32, 0, 1] {
            for dx in [-1i32, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = ((x as i32 + dx).rem_euclid(self.width as i32)) as usize;
                let ny = ((y as i32 + dy).rem_euclid(self.height as i32)) as usize;
                if self.cells[ny][nx].is_active() {
                    count += 1;
                }
            }
        }
        count
    }

    /// Count cells in the Alive state.
    pub fn population(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c == TritCell::Alive)
            .count()
    }

    /// Count cells in the Alive or Idle state.
    pub fn active_count(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|c| c.is_active())
            .count()
    }
}

// ---------------------------------------------------------------------------
// ThreeStateLife
// ---------------------------------------------------------------------------
// Rules (applied simultaneously to all cells):
//   Dead  -> Alive  if exactly 3 Alive neighbors
//   Alive -> Dead   if <2 or >3 Alive neighbors (overcrowded / isolated)
//   Alive -> Idle   if exactly 2 Alive neighbors (transitioning out)
//   Alive stays     if exactly 3 Alive neighbors
//   Idle  -> Alive  if 3 Alive neighbors
//   Idle  -> Dead   otherwise

pub struct ThreeStateLife {
    pub grid: LifeGrid,
}

impl ThreeStateLife {
    pub fn new(width: usize, height: usize) -> Self {
        ThreeStateLife {
            grid: LifeGrid::new(width, height),
        }
    }

    pub fn step(&mut self) {
        let w = self.grid.width;
        let h = self.grid.height;
        let mut next = vec![vec![TritCell::Dead; w]; h];

        for y in 0..h {
            for x in 0..w {
                let an = self.grid.alive_neighbors(x, y);
                let current = self.grid.get(x, y);
                next[y][x] = match current {
                    TritCell::Dead => {
                        if an == 3 {
                            TritCell::Alive
                        } else {
                            TritCell::Dead
                        }
                    }
                    TritCell::Alive => {
                        if an < 2 || an > 3 {
                            TritCell::Dead
                        } else if an == 2 {
                            TritCell::Idle
                        } else {
                            // an == 3
                            TritCell::Alive
                        }
                    }
                    TritCell::Idle => {
                        if an == 3 {
                            TritCell::Alive
                        } else {
                            TritCell::Dead
                        }
                    }
                };
            }
        }

        self.grid.cells = next;
        self.grid.generation += 1;
    }

    /// Set the given positions to Alive.
    pub fn set_pattern(&mut self, pattern: &[(usize, usize)]) {
        for &(x, y) in pattern {
            self.grid.set(x, y, TritCell::Alive);
        }
    }
}

// ---------------------------------------------------------------------------
// HighLifeTernary
// ---------------------------------------------------------------------------
// Born:    Dead -> Alive if alive_neighbors in {3, 6}
// Survive: Alive stays   if alive_neighbors in {2, 3}; else -> Idle
// Idle:    always -> Dead

pub struct HighLifeTernary {
    pub grid: LifeGrid,
}

impl HighLifeTernary {
    pub fn new(width: usize, height: usize) -> Self {
        HighLifeTernary {
            grid: LifeGrid::new(width, height),
        }
    }

    pub fn step(&mut self) {
        let w = self.grid.width;
        let h = self.grid.height;
        let mut next = vec![vec![TritCell::Dead; w]; h];

        for y in 0..h {
            for x in 0..w {
                let an = self.grid.alive_neighbors(x, y);
                let current = self.grid.get(x, y);
                next[y][x] = match current {
                    TritCell::Dead => {
                        if an == 3 || an == 6 {
                            TritCell::Alive
                        } else {
                            TritCell::Dead
                        }
                    }
                    TritCell::Alive => {
                        if an == 2 || an == 3 {
                            TritCell::Alive
                        } else {
                            TritCell::Idle
                        }
                    }
                    TritCell::Idle => TritCell::Dead,
                };
            }
        }

        self.grid.cells = next;
        self.grid.generation += 1;
    }

    /// Set the given positions to Alive.
    pub fn set_pattern(&mut self, pattern: &[(usize, usize)]) {
        for &(x, y) in pattern {
            self.grid.set(x, y, TritCell::Alive);
        }
    }
}

// ---------------------------------------------------------------------------
// DayAndNightTernary
// ---------------------------------------------------------------------------
// Born:    Dead  -> Alive if alive_neighbors  in {3,6,7,8}
//          Idle  -> Alive if active_neighbors in {3,6,7,8}
// Survive: Alive stays    if alive_neighbors  in {3,4,6,7,8}; else -> Idle
// Idle:    -> Alive if active_neighbors in {3,6,7,8}; else -> Dead

pub struct DayAndNightTernary {
    pub grid: LifeGrid,
}

impl DayAndNightTernary {
    pub fn new(width: usize, height: usize) -> Self {
        DayAndNightTernary {
            grid: LifeGrid::new(width, height),
        }
    }

    pub fn step(&mut self) {
        let w = self.grid.width;
        let h = self.grid.height;
        let mut next = vec![vec![TritCell::Dead; w]; h];
        const BIRTH_SET: [usize; 4] = [3, 6, 7, 8];
        const SURVIVE_SET: [usize; 5] = [3, 4, 6, 7, 8];

        for y in 0..h {
            for x in 0..w {
                let an = self.grid.alive_neighbors(x, y);
                let aact = self.grid.active_neighbors(x, y);
                let current = self.grid.get(x, y);
                next[y][x] = match current {
                    TritCell::Dead => {
                        if BIRTH_SET.contains(&an) {
                            TritCell::Alive
                        } else {
                            TritCell::Dead
                        }
                    }
                    TritCell::Alive => {
                        if SURVIVE_SET.contains(&an) {
                            TritCell::Alive
                        } else {
                            TritCell::Idle
                        }
                    }
                    TritCell::Idle => {
                        if BIRTH_SET.contains(&aact) {
                            TritCell::Alive
                        } else {
                            TritCell::Dead
                        }
                    }
                };
            }
        }

        self.grid.cells = next;
        self.grid.generation += 1;
    }

    /// Set the given positions to Alive.
    pub fn set_pattern(&mut self, pattern: &[(usize, usize)]) {
        for &(x, y) in pattern {
            self.grid.set(x, y, TritCell::Alive);
        }
    }
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

/// Given a population history, detect a repeating period from 2 to 10.
/// Returns Some(period) if the last 2*period values show periodic behavior.
pub fn detect_oscillation(history: &[usize]) -> Option<usize> {
    for period in 2..=10usize {
        let needed = period * 2;
        if history.len() < needed {
            continue;
        }
        let tail = &history[history.len() - needed..];
        let first_half = &tail[..period];
        let second_half = &tail[period..];
        if first_half == second_half {
            return Some(period);
        }
    }
    None
}

/// Returns true if the grid is a still life under ThreeStateLife rules
/// (i.e. applying one step yields the same configuration).
pub fn find_still_life(grid: &LifeGrid) -> bool {
    let mut sim = ThreeStateLife {
        grid: LifeGrid {
            width: grid.width,
            height: grid.height,
            cells: grid.cells.clone(),
            generation: grid.generation,
        },
    };
    let before = sim.grid.cells.clone();
    sim.step();
    sim.grid.cells == before
}

/// Statistics over a population history.
pub struct PopulationStats {
    pub min: usize,
    pub max: usize,
    pub mean: f64,
    pub variance: f64,
}

impl PopulationStats {
    pub fn compute(history: &[usize]) -> Self {
        if history.is_empty() {
            return PopulationStats {
                min: 0,
                max: 0,
                mean: 0.0,
                variance: 0.0,
            };
        }
        let min = *history.iter().min().unwrap();
        let max = *history.iter().max().unwrap();
        let n = history.len() as f64;
        let mean = history.iter().map(|&v| v as f64).sum::<f64>() / n;
        let variance = history
            .iter()
            .map(|&v| {
                let diff = v as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / n;
        PopulationStats {
            min,
            max,
            mean,
            variance,
        }
    }

    /// Returns true if variance is below 1.0 (population essentially constant).
    pub fn is_stable(&self) -> bool {
        self.variance < 1.0
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // 1. TritCell active/inactive
    #[test]
    fn trit_cell_active() {
        assert!(!TritCell::Dead.is_active());
        assert!(TritCell::Idle.is_active());
        assert!(TritCell::Alive.is_active());
    }

    // 2. Empty ThreeStateLife grid stays empty
    #[test]
    fn three_state_life_empty() {
        let mut sim = ThreeStateLife::new(10, 10);
        sim.step();
        assert_eq!(sim.grid.population(), 0);
        assert_eq!(sim.grid.active_count(), 0);
    }

    // 3. Dead cell with exactly 3 alive neighbors becomes Alive
    #[test]
    fn three_state_life_birth() {
        let mut sim = ThreeStateLife::new(10, 10);
        // Place 3 alive cells around (5,5)
        sim.grid.set(4, 4, TritCell::Alive);
        sim.grid.set(5, 4, TritCell::Alive);
        sim.grid.set(6, 4, TritCell::Alive);
        // (5,5) is dead and has exactly 3 alive neighbors
        assert_eq!(sim.grid.alive_neighbors(5, 5), 3);
        sim.step();
        assert_eq!(sim.grid.get(5, 5), TritCell::Alive);
    }

    // 4. Alive cell with 5 alive neighbors dies (overcrowding)
    #[test]
    fn three_state_life_death_overcrowded() {
        let mut sim = ThreeStateLife::new(10, 10);
        // Center cell
        sim.grid.set(5, 5, TritCell::Alive);
        // 5 neighbors
        sim.grid.set(4, 4, TritCell::Alive);
        sim.grid.set(5, 4, TritCell::Alive);
        sim.grid.set(6, 4, TritCell::Alive);
        sim.grid.set(4, 5, TritCell::Alive);
        sim.grid.set(6, 5, TritCell::Alive);
        assert_eq!(sim.grid.alive_neighbors(5, 5), 5);
        sim.step();
        assert_eq!(sim.grid.get(5, 5), TritCell::Dead);
    }

    // 5. Alive cell with 3 alive neighbors survives
    #[test]
    fn three_state_life_survives() {
        let mut sim = ThreeStateLife::new(10, 10);
        // A 2x2 block: each cell has exactly 3 Alive neighbors, all survive
        sim.grid.set(5, 5, TritCell::Alive);
        sim.grid.set(6, 5, TritCell::Alive);
        sim.grid.set(5, 6, TritCell::Alive);
        sim.grid.set(6, 6, TritCell::Alive);
        sim.step();
        assert_eq!(sim.grid.get(5, 5), TritCell::Alive);
        assert_eq!(sim.grid.get(6, 5), TritCell::Alive);
        assert_eq!(sim.grid.get(5, 6), TritCell::Alive);
        assert_eq!(sim.grid.get(6, 6), TritCell::Alive);
    }

    // 6. HighLife: dead cell with 6 alive neighbors is born
    #[test]
    fn highlife_born_on_six() {
        let mut sim = HighLifeTernary::new(10, 10);
        // Place 6 alive neighbors around (5,5)
        sim.grid.set(4, 4, TritCell::Alive);
        sim.grid.set(5, 4, TritCell::Alive);
        sim.grid.set(6, 4, TritCell::Alive);
        sim.grid.set(4, 5, TritCell::Alive);
        sim.grid.set(6, 5, TritCell::Alive);
        sim.grid.set(4, 6, TritCell::Alive);
        assert_eq!(sim.grid.alive_neighbors(5, 5), 6);
        sim.step();
        assert_eq!(sim.grid.get(5, 5), TritCell::Alive);
    }

    // 7. HighLife: alive cell with wrong neighbor count -> Idle, not Dead
    #[test]
    fn highlife_idle_transition() {
        let mut sim = HighLifeTernary::new(10, 10);
        // Lone alive cell, 0 neighbors -> should go to Idle
        sim.grid.set(5, 5, TritCell::Alive);
        assert_eq!(sim.grid.alive_neighbors(5, 5), 0);
        sim.step();
        assert_eq!(sim.grid.get(5, 5), TritCell::Idle);
    }

    // 8. DayAndNight: dead cell with 3 alive neighbors is born
    #[test]
    fn day_and_night_birth() {
        let mut sim = DayAndNightTernary::new(10, 10);
        sim.grid.set(4, 4, TritCell::Alive);
        sim.grid.set(5, 4, TritCell::Alive);
        sim.grid.set(6, 4, TritCell::Alive);
        assert_eq!(sim.grid.alive_neighbors(5, 5), 3);
        sim.step();
        assert_eq!(sim.grid.get(5, 5), TritCell::Alive);
    }

    // 9. detect_oscillation finds period 2 in alternating sequence
    #[test]
    fn detect_oscillation_period2() {
        let history = vec![10, 5, 10, 5, 10, 5];
        assert_eq!(detect_oscillation(&history), Some(2));
    }

    // 10. detect_oscillation returns None for monotonic history
    #[test]
    fn detect_oscillation_none() {
        let history = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(detect_oscillation(&history), None);
    }

    // 11. PopulationStats computes correct min/max/mean
    #[test]
    fn population_stats() {
        let history = vec![2, 4, 6, 8, 10];
        let stats = PopulationStats::compute(&history);
        assert_eq!(stats.min, 2);
        assert_eq!(stats.max, 10);
        assert!((stats.mean - 6.0).abs() < 1e-9);
    }

    // 12. Empty grid is a still life
    #[test]
    fn find_still_life_empty() {
        let grid = LifeGrid::new(8, 8);
        assert!(find_still_life(&grid));
    }
}
