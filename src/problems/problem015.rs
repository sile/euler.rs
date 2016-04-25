//! [15] Lattice paths
//! ------------------
//!
//! ### Problem
//! ```
//! Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
//!
//!     https://projecteuler.net/project/images/p015.gif
//!
//! How many such routes are there through a 20×20 grid?
//! ```
pub fn solve() -> u64 {
    const EDGES: usize = 20;
    let mut grid: [[u64; EDGES + 1]; EDGES + 1] = [[1; EDGES + 1]; EDGES + 1];

    for y in 1..grid.len() {
        for x in 1..grid.len() {
            grid[y][x] = grid[y][x - 1] + grid[y - 1][x];
        }
    }

    grid[EDGES][EDGES]
}
