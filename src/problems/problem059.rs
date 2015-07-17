//! [59] XOR decryption
//! -------------------
//!
use utils::{self,Sum};

pub fn solve() -> usize {
    let bytes = utils::load_bytes("data/p059_cipher.txt");

    // https://en.wikipedia.org/wiki/Most_common_words_in_English
    let common_words = vec!["the", "be", "to", "of", "and"];

    for a in 97..123 { // a..z
        for b in 97..123 {
            for c in 97..123 {
                let key = [a, b, c];
                let decrypted: Vec<_> = bytes.iter().zip(key.iter().cycle()).map(|(x, y)| x ^ y ).collect();
                let printable = decrypted.iter().all(|&x| is_printable(x) );
                if printable {
                    let original = String::from_utf8(decrypted).unwrap();
                    if common_words.iter().all(|w| original.contains(w) ) {
                        return original.bytes().map(|b| b as usize).summation()
                    }
                }
            }
        }
    }
    unreachable!()
}

fn is_printable(x: u8) -> bool {
    let ascii_printable_low = 0x20;
    let ascii_printable_high = 0x7E;
    ascii_printable_low <= x && x <= ascii_printable_high
}
