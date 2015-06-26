//! [12] Highly divisible triangular number
//! ---------------------------------------
//!
//! ### Problem
//! ```
//! The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
//!
//!     1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//!
//! Let us list the factors of the first seven triangle numbers:
//!
//!     1: 1
//!     3: 1,3
//!     6: 1,2,3,6
//!    10: 1,2,5,10
//!    15: 1,3,5,15
//!    21: 1,3,7,21
//!    28: 1,2,4,7,14,28
//!
//! We can see that 28 is the first triangle number to have over five divisors.
//! What is the value of the first triangle number to have over five hundred divisors?
//! ```
use utils::Sum;

pub fn solve() -> u64 {
    let divisors = 500;
    let mut triangle_num = 0;
    for n in (1..) {
        triangle_num += n;
        if (1..).take_while(|x| x*x <= triangle_num )
                .filter(|x| triangle_num % x == 0)
                .map(|x| if x*x == triangle_num { 1 } else { 2 })
                .summation() >= divisors
        {
            break;
        }
    }
    triangle_num
}