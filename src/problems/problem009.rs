//! [9] Special Pythagorean triplet
//! -------------------------------
//!
//! ### Problem
//! ```
//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//!
//!     a^2 + b^2 = c^2
//!
//! For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product abc.
//! ```
use num;

pub fn solve() -> usize {
    for c in 1..1000 {
        for b in 1..c {
            for a in 1..c {
                if a + b + c != 1000 {
                    continue;
                }
                if num::pow(a, 2) + num::pow(b, 2) == num::pow(c, 2) {
                    return a * b * c;
                }
            }
        }
    }
    unreachable!()
}
