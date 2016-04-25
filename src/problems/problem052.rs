//! [52] Permuted multiples
//! -----------------------
//!
//! https://projecteuler.net/problem=52
//!
pub fn solve() -> usize {
    (1..)
        .find(|&x| {
            let mut ds0 = [0; 10];
            let mut ds1 = [0; 10];
            count_digit(x, &mut ds0);
            (2..7).all(|y| {
                count_digit(x * y, &mut ds1);
                ds0 == ds1
            })
        })
        .unwrap()
}

fn count_digit(mut n: usize, ds: &mut [usize; 10]) {
    for i in 0..10 {
        ds[i] = 0;
    }
    while n != 0 {
        ds[n % 10] += 1;
        n /= 10;
    }
}
