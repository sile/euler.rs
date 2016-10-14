//! [16] Power digit sum
//! --------------------
//!
use num;
use num::bigint::BigUint;
use num::FromPrimitive;

pub fn solve() -> u64 {
    let power = 1000;
    format!("{}", num::pow(BigUint::from_u8(2).unwrap(), power))
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}
