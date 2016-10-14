//! [23] Non-abundant sums
//! -----------------
//!
use euler_lib::utils;

pub fn solve() -> u64 {
    let limit = 28123;
    let abundants = abundant_numbers(limit);
    (1..limit).filter(|&n| !is_writtern(n, &abundants)).sum::<usize>() as u64
}

fn abundant_numbers(limit: usize) -> Vec<usize> {
    (2..limit)
        .map(|x| (x, utils::divisors(x).sum()))
        .filter(|&(x, y)| x < y)
        .map(|(x, _)| x)
        .collect()
}

fn is_writtern(n: usize, abundants: &Vec<usize>) -> bool {
    abundants.iter()
        .filter(|&&x| x < n)
        .any(|&x| {
            let y = n - x;
            abundants.binary_search_by(|z| z.cmp(&y)).is_ok()
        })
}
