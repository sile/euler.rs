//! [36] Double-base palindromes
//! ----------------------------
//!
//! https://projecteuler.net/problem=36
//!
use utils::Sum;

pub fn solve() -> usize {
    (1..1_000_000).filter(|n| is_palindramic(&format!("{}", n)) && is_palindramic(&format!("{:b}", n)) ).summation()
}

fn is_palindramic(s: &str) -> bool {
    let s = s.as_bytes();
    (0..(s.len() / 2)).all(|i| s[i] == s[s.len()-i-1] )
}
