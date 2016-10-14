//! [28] Number spiral diagonals
//! ----------------------------
//!
//! https://projecteuler.net/problem=28
//!
pub fn solve() -> u64 {
    let mut edge_size = 1001;
    let mut n = edge_size * edge_size;
    let mut sum = 0;
    let mut phase = 0;
    while n > 1 {
        sum += n;
        n -= edge_size - 1;
        phase += 1;
        if phase == 4 {
            edge_size -= 2;
            phase = 0;
        }
    }
    sum += 1;
    sum as u64
}
