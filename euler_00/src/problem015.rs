//! [15] Lattice paths
//! ------------------
//!
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
