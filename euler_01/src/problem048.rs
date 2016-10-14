//! [48] Self powers
//! ----------------
//!
//! https://projecteuler.net/problem=48
//!
use euler_lib::utils::Sum;
use num;
use num::bigint::BigUint;
use num::FromPrimitive;
use num::traits::ToPrimitive;

pub fn solve() -> u64 {
    let n = (1..1001).map(|n| num::pow(BigUint::from_usize(n).unwrap(), n)).summation();
    (n % BigUint::from_usize(10_000_000_000).unwrap()).to_u64().unwrap()
}
