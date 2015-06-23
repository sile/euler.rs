//! [6] Sum square difference
//! -------------------------
//!
//! ### Problem
//! ```
//! The sum of the squares of the first ten natural numbers is,
//!     1^2 + 2^2 + ... + 10^2 = 385
//! The square of the sum of the first ten natural numbers is,
//!     (1 + 2 + ... + 10)^2 = 55^2 = 3025
//! Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//! Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
//! ```
extern crate num;

use utils::Sum;

pub fn solve() -> usize {
    num::pow((1..101).sumation(), 2) - (1..101).map(|n| num::pow(n,2)).sumation()
}
