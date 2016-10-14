//! [4] Largest palindrome product
//! ------------------------------
//!
pub fn solve() -> u64 {
    let mut max = 0;
    for x in 100..999 {
        for y in 100..999 {
            let n = x * y;
            if n > max && is_palindramic(n) {
                max = n
            }
        }
    }
    max as u64
}

fn is_palindramic(n: usize) -> bool {
    let s = format!("{}", n);
    let s = s.as_bytes();
    (0..(s.len() / 2)).all(|i| s[i] == s[s.len() - i - 1])
}
