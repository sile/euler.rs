//! [43] Sub-string divisibility
//! ----------------------------
//!
//! https://projecteuler.net/problem=43
//!
use euler_lib::utils;

struct Permutation {
    vec: Vec<u8>,
    nth: usize,
}

impl Iterator for Permutation {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        let r = utils::nth_permutation(&self.vec, self.nth);
        self.nth += 1;
        r
    }
}

fn permutation(vec: Vec<u8>) -> Permutation {
    Permutation { vec: vec, nth: 0 }
}

pub fn solve() -> u64 {
    let v = b"0123456789";
    permutation(v.iter().map(|&n| n).collect())
        .filter(|ds| ds[0] != 0)
        .filter(|ds| {
            (utils::digits_to_int(&ds[1..4]) % 2 == 0 && utils::digits_to_int(&ds[2..5]) % 3 == 0 &&
             utils::digits_to_int(&ds[3..6]) % 5 == 0 &&
             utils::digits_to_int(&ds[4..7]) % 7 == 0 &&
             utils::digits_to_int(&ds[5..8]) % 11 == 0 &&
             utils::digits_to_int(&ds[6..9]) % 13 == 0 &&
             utils::digits_to_int(&ds[7..10]) % 17 == 0)
        })
        .map(|ds| utils::digits_to_int(&ds) as u64)
        .sum()
}
