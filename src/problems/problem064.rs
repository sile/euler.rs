//! [64] Odd period square roots
//! ----------------------------
//!
// extern crate num;
// use self::num::integer;

#[derive(Debug)]
struct SquareRootFraction {
    n: usize,
    fs: usize,
    a: usize,
    num: usize,
    denom: usize,
}

impl Iterator for SquareRootFraction {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == self.fs * self.fs {
            return None
        }

        let curr = self.a;
        // num / (sqrt(n) - denom)
        // = (sqrt(n) + denom) / X
        // ----
        // X = (sqrt(n) + denom) / (num / (sqrt(n) - denom))
        //   = (sqrt(n) + denom) * (sqrt(n) - denom) / num
        //   = (sqrt(n)^2 - denom^2) / num
        //   = n - denom^2 / num
        // ---
        // (sqrt(n) + denom) / X = (sqrt(n) + denom) / (n - denom^2 / num)
        //                       = num * (sqrt(n) + denom) / (n - denom^2)
        // ---
        // invariant: (n - denom^2) % num == 0
        // ---
        // (sqrt(n) + denom) / ((n - denom^2) / num)
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

fn square_root_fractions(n: usize) -> SquareRootFraction {
    let fs = floor_square(n);
    SquareRootFraction{n: n, fs: fs, a: fs, num: 1, denom: fs}
}

pub fn solve() -> usize {
    for i in 2..10001 {
        println!("[{}] {:?}", i, square_root_fractions(i).take(10).collect::<Vec<_>>());
    }
    10
}

fn floor_square(n: usize) -> usize {
    (1..).take_while(|&i| i*i <= n ).last().unwrap()
}
