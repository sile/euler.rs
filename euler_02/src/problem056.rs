//! [56] Powerful digit sum
//! -----------------------
//!
//! https://projecteuler.net/problem=56
//!
use num;
use num::FromPrimitive;
use num::bigint::BigUint;

pub fn solve() -> u64 {
    (1..100)
        .map(|a| {
            (1..100)
                .map(|b| digital_sum(num::pow(BigUint::from_usize(a).unwrap(), b)))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn digital_sum(n: BigUint) -> u64 {
    format!("{}", n).chars().map(|c| c.to_digit(10).unwrap() as u64).sum()
}
