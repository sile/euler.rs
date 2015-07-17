//! [58] Spiral primes
//! ------------------
//!
use utils;

struct Diagonal {
    curr: u64,
    side: usize,
    rest_edges: usize,
}

impl Iterator for Diagonal {
    type Item = (usize, u64); // (side, curr)
    fn next(&mut self) -> Option<Self::Item> {
        let ret = (self.side, self.curr);
        if self.rest_edges == 0 {
            self.side += 2;
            self.rest_edges = 4;
        }
        self.curr += (self.side - 1) as u64;
        self.rest_edges -= 1;
        Some(ret)
    }
}

fn spiral_diagonals() -> Diagonal {
    Diagonal{curr: 1, side: 1, rest_edges: 0}
}

pub fn solve() -> usize {
    let mut diagonal_count = 1;
    let mut prime_count = 0;
    let mut primes = utils::primes().peekable();
    let mut last_side = 3;
    for (side, d) in spiral_diagonals().skip(1) {
        if side != last_side {
            if diagonal_count > prime_count * 10 {
                break;
            }
        }
        diagonal_count += 1;
        last_side = side;

        while d > *primes.peek().unwrap() {
            primes.next();
        }
        if d == *primes.peek().unwrap() {
            prime_count += 1;
        }
    }
    last_side
}
