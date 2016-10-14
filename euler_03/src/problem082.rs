//! [82] Path sum: three ways
//! -------------------------
//!
use euler_lib::utils;
use std::cmp;

const EDGE_SIZE: usize = 80;

pub fn solve() -> u64 {
    let matrix = utils::load_matrix("data/p082_matrix.txt", EDGE_SIZE, EDGE_SIZE);
    let mut table = matrix.clone();

    for col in 1..EDGE_SIZE {
        for row in 0..EDGE_SIZE {
            table[row][col] = table[row][col - 1] + matrix[row][col];
        }

        for _ in 0..EDGE_SIZE {
            for row in 0..EDGE_SIZE - 1 {
                table[row + 0][col] = cmp::min(table[row + 0][col],
                                               table[row + 1][col] + matrix[row + 0][col]);
                table[row + 1][col] = cmp::min(table[row + 1][col],
                                               table[row + 0][col] + matrix[row + 1][col]);
            }
        }
    }

    (0..EDGE_SIZE).map(|i| table[i][EDGE_SIZE - 1]).min().unwrap() as u64
}
