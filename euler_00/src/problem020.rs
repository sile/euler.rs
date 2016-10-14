//! [20] Factorial digit sum
//! ------------------------
//!
use num::bigint::BigUint;
use euler_lib::utils;

pub fn solve() -> u64 {
    let num = 100;
    format!("{}", utils::factorial::<BigUint>(num))
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}
