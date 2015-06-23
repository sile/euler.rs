//! [4] Largest palindrome product
//! ------------------------------
//!
//! ### Problem
//! ```
//! A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//! Find the largest palindrome made from the product of two 3-digit numbers.
//! ```
pub fn solve() -> usize {
    let mut max = 0;
    for x in 100..999 {
        for y in 100..999 {
            let n = x * y;
            if n > max && is_palindramic(n) {
                max = n
            }
        }
    }
    max
}

fn is_palindramic(n: usize) -> bool {
    let s = format!("{}", n);
    let s = s.as_bytes();
    (0..(s.len() / 2)).all(|i| s[i] == s[s.len()-i-1] )
}
