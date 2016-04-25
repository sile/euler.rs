//! [39] Integer right triangles
//! -------------------------
//!
//! https://projecteuler.net/problem=39
//!
pub fn solve() -> usize {
    (3..1001)
        .map(|p| (integer_right_triangles(p), p))
        .max()
        .unwrap()
        .1
}

fn integer_right_triangles(p: usize) -> usize {
    let mut count = 0;
    for x in 1..p {
        for y in 1..(p - x) {
            let z = p - x - y;
            if x * x == y * y + z * z {
                count += 1;
            }
        }
    }
    count
}
