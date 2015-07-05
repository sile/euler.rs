//! [41] Pandigital prime
//! ---------------------
//!
//! https://projecteuler.net/problem=41
//!
use utils;

pub fn solve() -> u64 {
    utils::primes()
        .take_while(|&n| n <= 987654321 )
        .filter(|&n| is_pandigital(n) )
        .last().unwrap()
}

fn is_pandigital(mut n: u64) -> bool {
    let mut digits = [false; 9];
    let mut count = 0;
    while n > 0 {
        let x = (n % 10) as usize;
        if x == 0 || digits[x-1] { return false }

        count += 1;
        digits[x-1] = true;
        n /= 10;
    }
    digits[0..count].iter().all(|&b| b )
}
