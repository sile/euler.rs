//! [33] Digit cancelling fractions
//! -------------------------------
//!
//! https://projecteuler.net/problem=33
//!
use num::rational::Ratio;

pub fn solve() -> usize {
    let mut r = Ratio::from_integer(1);
    for x in 1..10 {
        for y in 1..10 {
            for z in 1..10 {
                let n = Ratio::new(y, z);
                if n >= Ratio::from_integer(1) {
                    continue;
                }

                if n == Ratio::new(x * 10 + y, x * 10 + z) ||
                   n == Ratio::new(x * 10 + y, z * 10 + x) ||
                   n == Ratio::new(y * 10 + x, x * 10 + z) ||
                   n == Ratio::new(y * 10 + x, z * 10 + x) {
                    r = r * n;
                }
            }
        }
    }
    *r.denom()
}
