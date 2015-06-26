//! [20] Factorial digit sum
//! ------------------------
//!
//! ### Problem
//! ```
//! n! means n × (n − 1) × ... × 3 × 2 × 1
//!
//! For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
//! and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//!
//! Find the sum of the digits in the number 100!
//! ```
extern crate num;
use self::num::bigint::BigUint;
use self::num::FromPrimitive;
use self::num::One;
use utils::Sum;

pub fn solve() -> u32 {
    let num = 100;
    format!("{}", factorial(num)).chars().map(|c| c.to_digit(10).unwrap() ).summation()
}

fn factorial(n: usize) -> BigUint {
    (1..n+1).fold(BigUint::one(), |a,b| a * BigUint::from_usize(b).unwrap() )
}
