//! [5] Smallest multiple
//! ---------------------
//!
//! ### Problem
//! ```
//! 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
//! ```
pub fn solve() -> usize {
    (1..).filter(|x| (1..21).all(|y| x % y == 0)).nth(0).unwrap()
}
