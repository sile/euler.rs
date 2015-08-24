//! [83] Path sum: four ways
//! ------------------------
//!
use utils;
use std::usize;

const EDGE_SIZE: usize = 80;
const AREA_SIZE: usize = EDGE_SIZE * EDGE_SIZE;
const INFINITY: usize = usize::MAX / 2;

fn i(row: usize, col: usize) -> usize {
    row * EDGE_SIZE + col
}

pub fn solve() -> usize {
    let matrix = utils::load_matrix("data/p083_matrix.txt", EDGE_SIZE, EDGE_SIZE);
    let mut table = vec![vec![INFINITY; AREA_SIZE]; AREA_SIZE];

    // "Floyd-Warshall's shortest path algorithm" based solution

    // self
    for row in 0..EDGE_SIZE {
        for col in 0..EDGE_SIZE {
            table[i(row,col)][i(row,col)] = matrix[row][col];
        }
    }

    // adjucent
    for row in 0..EDGE_SIZE {
        for col in 0..EDGE_SIZE {
            if row > 0 {
                table[i(row,col)][i(row-1,col)] = matrix[row][col] + matrix[row-1][col];
            }
            if row < EDGE_SIZE-1 {
                table[i(row,col)][i(row+1,col)] = matrix[row][col] + matrix[row+1][col];
            }
            if col > 0 {
                table[i(row,col)][i(row,col-1)] = matrix[row][col] + matrix[row][col-1];
            }
            if col < EDGE_SIZE-1 {
                table[i(row,col)][i(row,col+1)] = matrix[row][col] + matrix[row][col+1];
            }
        }
    }

    // others
    for pivot in 0..AREA_SIZE {
        println!("    {}/{}", pivot+1, AREA_SIZE);
        for from in 0..AREA_SIZE {
            if table[from][pivot] == INFINITY { continue }
            for to in 0..AREA_SIZE {
                if table[from][pivot] + table[pivot][to] - table[pivot][pivot] < table[from][to] {
                    table[from][to] = table[from][pivot] + table[pivot][to] - table[pivot][pivot];
                }
            }
        }
    }

    table[0][AREA_SIZE-1]
}
