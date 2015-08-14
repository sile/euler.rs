//! [56] Powerful digit sum
//! -----------------------
//!
//! https://projecteuler.net/problem=56
//!
use num;
use num::FromPrimitive;
use num::bigint::BigUint;
use utils::Sum;

pub fn solve() -> usize {
    (1..100).map(|a| {
        (1..100).map(|b| digital_sum(num::pow(BigUint::from_usize(a).unwrap(), b)) ).max().unwrap()
    } ).max().unwrap()
}

fn digital_sum(n: BigUint) -> usize {
    format!("{}", n).chars().map(|c| c.to_digit(10).unwrap() ).summation() as usize
}
