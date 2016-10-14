//! [24] Lexicographic permutations
//! -------------------------------
//!
use euler_lib::utils;

pub fn solve() -> u64 {
    let v = b"0123456789";
    let nth = 1_000_000;
    utils::nth_permutation(v, nth - 1).unwrap().iter().fold(0, |a, &b| a * 10 + b as u64)
}
