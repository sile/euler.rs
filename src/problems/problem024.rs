//! [24] Lexicographic permutations
//! -------------------------------
//!
//! ### Problem
//! ```
//! A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all
//! of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1
//! and 2 are:
//!
//!     012   021   102   120   201   210
//!
//! What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
//! ```
use utils;

pub fn solve() -> usize {
    let v = b"0123456789";
    let nth = 1_000_000;
    nth_permutation(v, nth - 1).iter().fold(0, |a,&b| a*10+b)
}

fn nth_permutation(v: &[u8], mut nth: usize) -> Vec<usize> {
    let mut r = Vec::new();
    for i in 0..v.len() {
        let f = utils::factorial(v.len() - i - 1);
        for j in 0..v.len() {
            if r.contains(&j) {
                continue;
            }
            if nth < f {
                r.push(j);
                break;
            }
            nth -= f;
        }
    }
    r
}
