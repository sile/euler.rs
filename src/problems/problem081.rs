//! [81] Path sum: two ways
//! -----------------------
//!
use utils;
use std::usize;
use std::cmp;

const EDGE_SIZE: usize = 80;

pub fn solve() -> usize {
    let matrix = utils::load_matrix("data/p081_matrix.txt", EDGE_SIZE, EDGE_SIZE);
    let mut table = [[usize::MAX; EDGE_SIZE + 1]; EDGE_SIZE + 1];
    table[0][1] = 0;
    table[1][0] = 0;

    for row in 1..EDGE_SIZE + 1 {
        for col in 1..EDGE_SIZE + 1 {
            table[row][col] = matrix[row - 1][col - 1] +
                              cmp::min(table[row - 1][col], table[row][col - 1]);
        }
    }

    table[EDGE_SIZE][EDGE_SIZE]
}
