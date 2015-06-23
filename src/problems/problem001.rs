//! [1] Multiples of 3 and 5
//! ------------------------
//!
//! ### Problem
//! ```
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//! Find the sum of all the multiples of 3 or 5 below 1000.
//! ```
use utils::Sum;

pub fn solve() -> usize {
    let limit = 1000;
    (1..limit).filter(|n| n % 3 == 0 || n % 5 == 0).summation()
}
