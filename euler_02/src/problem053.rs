//! [53] Combinatoric selections
//! ----------------------------
//!
//! https://projecteuler.net/problem=53
//!
pub fn solve() -> u64 {
    let mut count = 0;
    for n in 1..101 {
        for r in 1..n + 1 {
            if is_combinatric_greater_than(n, r, 1_000_000) {
                count += 1;
            }
        }
    }
    count
}

fn is_combinatric_greater_than(n: usize, r: usize, border: usize) -> bool {
    let mut nums = r + 1..n + 1;
    let mut denoms = 1..(n - r) + 1;
    let mut tmp = 1.0 as f64;

    loop {
        if tmp > 1.0 {
            if let Some(x) = denoms.next() {
                tmp /= x as f64;
                continue;
            }
        }

        if let Some(x) = nums.next() {
            tmp *= x as f64;
            if tmp >= border as f64 {
                break;
            }
        } else {
            break;
        }
    }
    tmp >= border as f64
}
