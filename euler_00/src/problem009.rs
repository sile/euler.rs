//! [9] Special Pythagorean triplet
//! -------------------------------
//!
use num;

pub fn solve() -> u64 {
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
