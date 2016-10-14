//! [80] Square root digital expansion
//! ----------------------------------
//!
use num::{self, FromPrimitive};
use num::rational::{Ratio, BigRational};
use num::bigint::BigInt;

struct Sqrt {
    n: BigRational,
    cur: BigRational,
    i: usize,
}

impl Iterator for Sqrt {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let denom = num::pow(BigInt::from_u64(10).unwrap(), self.i);
        let mut d = 0;
        for num in 1.. {
            let tmp = num::pow(&self.cur +
                               Ratio::new(BigInt::from_usize(num).unwrap(), denom.clone()),
                               2);
            if self.n < tmp {
                d = num - 1;
                break;
            } else if self.n == tmp {
                return None;
            }
        }
        self.i += 1;
        self.cur = &self.cur + Ratio::new(BigInt::from_usize(d).unwrap(), denom);
        Some(d as u64)
    }
}

fn sqrt(n: usize) -> Sqrt {
    Sqrt {
        n: Ratio::from_integer(BigInt::from_usize(n).unwrap()),
        cur: Ratio::from_integer(num::zero()),
        i: 0,
    }
}

pub fn solve() -> u64 {
    (1..101)
        .map(|n| (n, sqrt(n).take(100).collect::<Vec<_>>()))
        .filter(|x| x.1.len() == 100)
        .map(|x| (x.0, x.1.into_iter().sum::<u64>()))
        .inspect(|x| println!("    {}: {}", x.0, x.1))
        .map(|x| x.1)
        .sum()
}
