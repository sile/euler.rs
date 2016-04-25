//! [47] Distinct primes factors
//! ----------------------------
//!
//! https://projecteuler.net/problem=47
//!
use utils;

pub fn solve() -> u64 {
    let mut primes = vec![2];
    let mut buf = vec![0; 4];
    utils::composites()
        .filter(|&n| prime_factors(n, &mut primes) == 4)
        .find(|&n| {
            buf.insert(0, n);
            buf.pop();
            buf[0..buf.len() - 1].iter().zip(buf[1..buf.len()].iter()).all(|(&x, &y)| x - y == 1)
        })
        .unwrap() - 3
}

fn prime_factors(mut n: u64, primes: &mut Vec<u64>) -> usize {
    if n > *primes.last().unwrap() {
        *primes = utils::primes().take_while(|&p| p <= n * 2).collect();
    }

    let mut count = 0;
    for &p in primes.iter() {
        if n == 1 {
            break;
        }
        if n % p != 0 {
            continue;
        }
        while n % p == 0 {
            n /= p
        }
        count += 1;
    }
    count
}
