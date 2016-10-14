//! [87] Prime power triples
//!
//!
use std::collections::HashSet;
use euler_lib::utils;

pub fn solve() -> u64 {
    let limit = 50_000_000;
    let primes = utils::primes().take_while(|p| p * p < limit).collect::<Vec<_>>();
    let mut knowns = HashSet::new();
    for a in &primes {
        let aa = a * a;
        for b in &primes {
            let bbb = b * b * b;
            if aa + bbb >= limit {
                break;
            }
            for c in &primes {
                let cccc = c * c * c * c;
                if aa + bbb + cccc >= limit {
                    break;
                }
                knowns.insert(aa + bbb + cccc);
            }
        }
    }
    knowns.len() as u64
}
