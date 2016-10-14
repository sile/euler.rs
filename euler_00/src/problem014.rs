//! [14] Longest Collatz sequence
//! -----------------------------
//!
struct Collatz(usize);

impl Iterator for Collatz {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.0 == 1 {
            return None;
        }

        let n = self.0;
        self.0 = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        Some(n)
    }
}

pub fn solve() -> u64 {
    let limit = 1_000_000;
    (1..limit).map(|i| (Collatz(i).count(), i)).max().unwrap().1 as u64
}
