//! [17] Number letter counts
//! -------------------------
//!
//! ### Problem
//! ```
//! If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
//!
//! If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
//!
//! NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.
//! ```
use utils::Sum;

pub fn solve() -> usize {
    let max = 1000;
    (1..max+1).map(|n| word(n).chars().filter(|c| c.is_alphabetic() ).count() ).summation()
}

fn word(n: usize) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        1000 => "one thousand".to_string(),
        _  => {
            if n >= 100 {
                let low = n % 100;
                if low == 0 {
                    word(n / 100) + " hundred"
                } else {
                    word(n / 100) + " hundred and " + &word(low)
                }
            } else if n > 20 {
                word(n / 10 * 10) + "-" + &word(n % 10)
            } else {
                unimplemented!()
            }
        },
    }
}
