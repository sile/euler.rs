//! [16] Power digit sum
//! --------------------
//!
//! ### Problem
//! ```
//! 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//!
//! What is the sum of the digits of the number 2^1000?
//! ```
use num;
use num::bigint::BigUint;
use num::FromPrimitive;
use utils::Sum;

pub fn solve() -> u32 {
    let power = 1000;
    format!("{}", num::pow(BigUint::from_u8(2).unwrap(), power)).chars().map(|c| c.to_digit(10).unwrap() ).summation()
}
