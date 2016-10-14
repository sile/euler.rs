//! [49] Prime permutations
//! -----------------------
//!
//! https://projecteuler.net/problem=49
//!
use euler_lib::utils;
use std::collections::HashMap;

pub fn solve() -> u64 {
    let mut primes = HashMap::new(); // prime => permutations(prime)

    for p in utils::primes() {
        let d = utils::digits(p);
        if d < 4 {
            continue;
        }
        if d > 4 {
            break;
        }

        let least = least_permutation(p);
        if !primes.contains_key(&least) {
            primes.insert(least, vec![]);
        }
        primes.get_mut(&least).unwrap().push(p);
    }

    primes.values()
        .filter(|ps| ps.len() >= 3)
        .flat_map(|ps| {
            let mut v = Vec::new();
            for i in 0..(ps.len() - 2) {
                for j in (i + 1)..(ps.len() - 1) {
                    for k in (j + 1)..ps.len() {
                        v.push(vec![ps[i], ps[j], ps[k]]);
                    }
                }
            }
            v
        })
        .filter(|ps| ps[2] - ps[1] == ps[1] - ps[0])
        .filter(|ps| ps[0] != 1487)
        .map(|ps| ps[0] * 10000_0000 + ps[1] * 10000 + ps[2])
        .nth(0)
        .unwrap()
}

fn least_permutation(n: u64) -> u64 {
    let mut ds = utils::int_to_digits(n as usize);
    ds.sort();
    utils::digits_to_int(&ds) as u64
}
