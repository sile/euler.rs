//! [87] Product-sum numbers
//!
//!
use std::cmp;
use std::collections::HashSet;

pub fn solve() -> u64 {
    let mut numbers = HashSet::new();
    for n in 2..12000 {
        println!("{}", n);
        numbers.insert(minimal_product_sum(n));
    }
    numbers.into_iter().sum()
}

fn minimal_product_sum(k: u64) -> u64 {
    (k + 1..).find(|&n| is_product_sum(n, n, k)).unwrap()
}

fn is_product_sum(prod: u64, sum: u64, k: u64) -> bool {
    if prod == 1 && sum == k {
        return true;
    }
    if k == 0 {
        return false;
    }
    (2..cmp::min(prod, sum) + 1)
        .filter(|&x| prod % x == 0)
        .find(|&x| is_product_sum(prod / x, sum - x, k - 1))
        .is_some()
}
