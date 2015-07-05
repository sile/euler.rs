extern crate num;

use std::ops::{Add,Mul,Div};
use std::collections::HashMap;
use self::num::{One,Zero,FromPrimitive};

pub struct Fibonacci<T> {
    curr: T,
    next: T,
}

impl<T: Add<Output=T>+One+Clone> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let new_next = self.curr.clone() + self.next.clone();
        self.curr = self.next.clone();
        self.next = new_next;
        Some(self.curr.clone())
    }
}

pub fn fibonacci<T: One>() -> Fibonacci<T> {
    Fibonacci {curr: num::one(), next: num::one()}
}

pub struct Prime {
    curr: u64,
    sieve: HashMap<u64, u64>, // composite_number => prime_number
}

impl Prime {
    pub fn is_prime(&mut self, n: u64) -> bool {
        match self.sieve.remove(&n) {
            None        => {self.mark_composite(2, n);               true},  // prime number
            Some(prime) => {self.mark_composite(n/prime + 1, prime); false}, // composite number
        }
    }
    fn mark_composite(&mut self, times: u64, prime: u64) {
        (times..).find(|i| !self.sieve.contains_key(&(i*prime)) ).and_then(|i| self.sieve.insert(i*prime, prime) );
    }
}

impl Iterator for Prime {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let prime = (self.curr..).find(|n| self.is_prime(*n) ).unwrap();
        self.curr = prime + 1;
        Some(prime)
    }
}

pub fn primes() -> Prime {
    Prime{curr: 2, sieve: HashMap::new()}
}

pub trait Sum<T> {
    fn summation(self) -> T;
}

impl<T,O> Sum<O> for T where T: Iterator<Item=O>, O: Add + Zero {
    fn summation(self) -> O {
        self.fold(num::zero(), |a,b| a+b)
    }
}

pub trait Product<T> {
    fn prod(self) -> T;
}

impl<T,O> Product<O> for T where T: Iterator<Item=O>, O: Mul + One {
    fn prod(self) -> O {
        self.fold(num::one(), |a,b| a*b)
    }
}

pub struct Divisor {
    dividend: usize,
    curr: usize,
    buff: Option<usize>,
}

impl Iterator for Divisor {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if let Some(n) = self.buff {
            self.buff = None;
            return Some(n);
        }
        while self.dividend % self.curr != 0 {
            self.curr += 1;
        }
        if self.curr * self.curr > self.dividend {
            return None;
        }
        let x = self.curr;
        let y = self.dividend / x;
        self.curr += 1;
        if x != y {
            self.buff = Some(y);
        }
        Some(x)
    }
}

pub fn divisors(dividend: usize) -> Divisor {
    Divisor{dividend: dividend, curr: 2, buff: Some(1)}
}

pub fn factorial<T: One+Add<Output=T>+Mul+Clone>(i: usize) -> T {
    let mut n: T = num::one();
    let mut c: T = num::one();
    for _ in (0..i) {
        n = n * c.clone();
        c = c + num::one();
    }
    n
}

pub fn digits<T: FromPrimitive+Div<Output=T>+Zero+Eq+Clone>(mut n: T) -> usize {
    let mut d = 0;
    let ten = T::from_u8(10).unwrap();
    while n != num::zero() {
        n = n / ten.clone();
        d += 1
    }
    d
}

pub fn int_to_digits(mut n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    v
}
