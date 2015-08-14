//! [48] Self powers
//! ----------------
//!
//! https://projecteuler.net/problem=48
//!
use utils::Sum;
use num;
use num::bigint::BigUint;
use num::FromPrimitive;

pub fn solve() -> BigUint {
    let n = (1..1001).map(|n| num::pow(BigUint::from_usize(n).unwrap(), n) ).summation();
    n % BigUint::from_usize(10_000_000_000).unwrap()
}
