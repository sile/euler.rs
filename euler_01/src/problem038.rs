//! [38] Pandigital multiples
//! -------------------------
//!
//! https://projecteuler.net/problem=38
//!
use num;
use euler_lib::utils;

pub fn solve() -> u64 {
    (2..)
        .take_while(|&n| n * num::pow(10, utils::digits(n * 2)) + n * 2 <= 987654321)
        .filter_map(|n| pandigital(n))
        .max()
        .unwrap()
}

const UNDEF: u8 = 255;
fn pandigital(n: u64) -> Option<u64> {
    let mut digits: [u8; 9] = [UNDEF; 9];
    let mut j = 9;
    for i in 1.. {
        if digits.iter().all(|&n| n != UNDEF) {
            break;
        }
        let mut x = n * i;

        let k = utils::digits(x) as u8;
        if j < k {
            return None;
        }

        j -= k;
        for l in 0..k {
            let (y, z) = num::integer::div_rem(x, 10);
            if z == 0 || digits[z as usize - 1] != UNDEF {
                return None;
            }
            digits[z as usize - 1] = j + l;
            x = y;
        }
    }
    Some((1..10).zip(digits.iter()).fold(0, |a, (x, &i)| a + x * num::pow(10, i as usize)))
}
