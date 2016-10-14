//! [2] Even Fibonacci numbers
//! --------------------------
//!
use num::integer::Integer;
use euler_lib::utils;

pub fn solve() -> u64 {
    let limit = 4_000_000;
    utils::fibonacci::<u64>().take_while(|&n| n <= limit).filter(|n| n.is_even()).sum()
}
