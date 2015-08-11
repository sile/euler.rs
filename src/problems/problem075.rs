//! [75] Singular integer right triangles
//! -------------------------------------
//!
use std::collections::HashSet;

struct PrimitivePythagoreanTriple {
    n: usize,
    m: usize,
    limit: usize,
}

impl Iterator for PrimitivePythagoreanTriple {
    type Item = (usize, usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (a, b, c) = self.triple();
        if a + b + c <= self.limit {
            self.m += 2;
            Some ((a, b, c))
        } else {
            self.n += 1;
            self.m = self.n + 1;
            let (a, b, c) = self.triple();
            if a + b + c <= self.limit {
                self.m += 2;
                Some ((a, b, c))
            } else {
                None
            }
        }
    }
}

impl PrimitivePythagoreanTriple {
    fn triple(&self) -> (usize, usize, usize) {
        let a = self.m * self.m - self.n * self.n;
        let b = 2 * self.m * self.n;
        let c = self.m * self.m + self.n * self.n;
        (a, b, c)
    }
}

fn primitive_pythagorean_triples(limit: usize) -> PrimitivePythagoreanTriple {
    PrimitivePythagoreanTriple{n: 1, m: 2, limit: limit}
}

pub fn solve() -> usize {
    let limit = 1_500_000;
    let mut triangles = HashSet::new();
    for (a, b, c) in primitive_pythagorean_triples(limit) {
        let l = a + b + c;
        let mut n = 1;
        while l * n  <= limit {
            triangles.insert( (a*n, b*n, c*n) );
            n += 1;
        }
    }
    let mut counts = vec![0; limit + 1];
    for (a, b, c) in triangles.into_iter() {
        let l = a + b + c;
        counts[l] += 1;
    }
    counts.into_iter().filter(|&n| n == 1 ).count()
}
