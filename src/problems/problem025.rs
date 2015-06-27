//! [25] 1000-digit Fibonacci number
//! --------------------------------
//!
//! ### Problem
//! ```
//! The Fibonacci sequence is defined by the recurrence relation:
//!
//! F(n) = F(n-1) + F(n-2) where F(1) = 1 and F(2) = 1
//!
//! Hence the first 12 terms will be:
//!
//!     F(1) = 1
//!     F(2) = 1
//!     F(3) = 2
//!     F(4) = 3
//!     F(5) = 5
//!     F(6) = 8
//!     F(7) = 13
//!     F(8) = 21
//!     F(9) = 34
//!     F(10) = 55
//!     F(11) = 89
//!     F(12) = 144
//!
//! The 12th term, F(12), is the first term to contain three digits.
//!
//! What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
//! ```
extern crate num;

use utils;
use self::num::bigint::BigUint;
use self::num::One;

pub fn solve() -> usize {
    let border = n_digits(1000);
    utils::fibonacci::<BigUint>().position(|n| n >= border ).unwrap() + 2 // +1 for 1-based indexing. +1 for the fibonacci sequence where f(1)==1, f(2)==2, ...
}

fn n_digits(n: usize) -> BigUint {
    let ten = BigUint::parse_bytes(b"10", 10).unwrap();
    (1..n).fold(BigUint::one(), |a,_| a*&ten )
}
