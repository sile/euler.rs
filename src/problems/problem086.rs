//! [86] Cuboid route
//!
//!
use num::rational::Ratio;

pub fn solve() -> usize {
    let mut count = 0;
    for x in 1.. {
        for y in 1..(x + 1) {
            for z in 1..(y + 1) {
                let distance = shortest_path(x, y, z);
                if maybe_integer(distance) {
                    count += 1;
                    if count >= 1_000_000 {
                        return x;
                    }
                }
            }
        }
    }
    unreachable!()
}

fn maybe_integer(x: f64) -> bool {
    (x.round() - x).abs() < 0.0001
}

fn shortest_path(x: usize, y: usize, z: usize) -> f64 {
    let x1 = Ratio::new(y * x, y + z);
    let x2 = Ratio::new(z * x, y + z);
    let yy = Ratio::from_integer(y * y);
    let zz = Ratio::from_integer(z * z);
    to_float(yy + x1 * x1).sqrt() + to_float(zz + x2 * x2).sqrt()
}

fn to_float(x: Ratio<usize>) -> f64 {
    *x.numer() as f64 / *x.denom() as f64
}
