//! [80] Square root digital expansion
//! ----------------------------------
//!
use num::{self, FromPrimitive};
use num::rational::{Ratio, BigRational};
use num::bigint::BigInt;
use utils::Sum;

struct Sqrt {
    n: BigRational,
    cur: BigRational,
    i: usize,
}

impl Iterator for Sqrt {
    type Item = usize;
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
        Some(d)
    }
}

fn sqrt(n: usize) -> Sqrt {
    Sqrt {
        n: Ratio::from_integer(BigInt::from_usize(n).unwrap()),
        cur: Ratio::from_integer(num::zero()),
        i: 0,
    }
}

pub fn solve() -> usize {
    (1..101)
        .map(|n| (n, sqrt(n).take(100).collect::<Vec<_>>()))
        .filter(|x| x.1.len() == 100)
        .map(|x| (x.0, x.1.into_iter().summation()))
        .inspect(|x| println!("    {}: {}", x.0, x.1))
        .map(|x| x.1)
        .summation()
}
