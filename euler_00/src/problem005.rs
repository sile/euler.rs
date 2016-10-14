//! [5] Smallest multiple
//! ---------------------
//!
pub fn solve() -> u64 {
    (1..).filter(|x| (1..21).all(|y| x % y == 0)).nth(0).unwrap()
}
