//! [1] Multiples of 3 and 5
//! ------------------------
//!
pub fn solve() -> u64 {
    let limit = 1000;
    (1..limit).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}
