//! [66] Diophantine equation
//! -------------------------
//!
//! see also: https://en.wikipedia.org/wiki/Pell%27s_equation
extern crate num;

use self::num::rational::Ratio;
use self::num::bigint::BigUint;
use self::num::{Zero,One,FromPrimitive};

// see also: problem064.rs
#[derive(Debug)]
struct SquareRootFraction {
    n: u64,
    fs: u64,
    a: u64,
    num: u64,
    denom: u64,
}

impl Iterator for SquareRootFraction {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == self.fs * self.fs {
            return None
        }

        let curr = self.a;
        let denom = self.n - self.denom * self.denom;
        assert!(denom % self.num == 0);
        let denom = denom / self.num;

        let a = (self.fs + self.denom) / denom;
        let num = denom * a - self.denom;

        self.a = a;
        self.num = denom;
        self.denom = num;

        Some(curr)
    }
}

fn square_root_fractions(n: u64) -> SquareRootFraction {
    let fs = floor_square(n);
    SquareRootFraction{n: n, fs: fs, a: fs, num: 1, denom: fs}
}

fn floor_square(n: u64) -> u64 {
    (1..).take_while(|&i| i*i <= n ).last().unwrap()
}

pub fn solve() -> u64 {
    (2..1001)
        .map(|n| solve_quadratic_diophantine_equation(n) )
        .inspect(|t| println!("    [{}] x={}, y={}", t.2, t.0, t.1 ) )
        .max()
        .unwrap().2
}

fn solve_quadratic_diophantine_equation(d: u64) -> (BigUint, BigUint, u64) {
    let mut is = Vec::new();
    for i in square_root_fractions(d) {
        let mut sqrt = Ratio::from_integer(BigUint::from_u64(i).unwrap());
        for j in 0..is.len() {
            let n = is[is.len()-j-1];
            sqrt = Ratio::from_integer(BigUint::one()) / sqrt;
            sqrt = Ratio::from_integer(BigUint::from_u64(n).unwrap()) + sqrt;
        }
        let x = sqrt.numer().clone();
        let y = sqrt.denom().clone();
        let xx = num::pow(x.clone(), 2);
        let yy = num::pow(y.clone(), 2);
        if xx == BigUint::one() + yy * BigUint::from_u64(d).unwrap() {
            return (x, y, d)
        }
        is.push(i);
    }
    (BigUint::zero(), BigUint::zero(), d) // d is a square number
}
