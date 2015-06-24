extern crate num;

use std::ops::{Add,Mul};
use std::collections::HashMap;

pub struct Fibonacci {
    curr: usize,
    next: usize,
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

pub fn fibonacci() -> Fibonacci {
    Fibonacci {curr: 1, next: 1}
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

impl<T,O> Sum<O> for T where T: Iterator<Item=O>, O: Add + num::Zero {
    fn summation(self) -> O {
        self.fold(num::zero(), |a,b| a+b)
    }
}

pub trait Product<T> {
    fn prod(self) -> T;
}

impl<T,O> Product<O> for T where T: Iterator<Item=O>, O: Mul + num::One {
    fn prod(self) -> O {
        self.fold(num::one(), |a,b| a*b)
    }
}
