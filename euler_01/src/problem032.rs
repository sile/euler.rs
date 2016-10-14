//! [32] Pandigital products
//! ------------------------
//!
//! https://projecteuler.net/problem=32
//!
use std::collections::HashSet;

pub fn solve() -> u64 {
    let mut digits: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut perms = Vec::new();
    let mut pandigital_products: HashSet<usize> = HashSet::new();

    permtations(0, &mut digits, &mut perms);
    for v in perms.iter() {
        for i in 1..7 {
            for j in i + 1..8 {
                let x = (0..i).fold(0, |a, b| a * 10 + v[b]);
                let y = (i..j).fold(0, |a, b| a * 10 + v[b]);
                let z = (j..9).fold(0, |a, b| a * 10 + v[b]);
                if x * y == z {
                    pandigital_products.insert(z);
                }
            }
        }
    }
    pandigital_products.iter().map(|&n| n as u64).sum()
}

fn permtations(i: usize, digits: &mut [usize], out: &mut Vec<Vec<usize>>) {
    if digits.len() < i + 2 {
        out.push(digits.iter().map(|&n| n).collect());
        return;
    }

    for j in 0..digits.len() - i {
        digits.swap(i, i + j);
        permtations(i + 1, digits, out);
        digits.swap(i, i + j);
    }
}
