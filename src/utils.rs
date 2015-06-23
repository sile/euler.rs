extern crate num;

use std::ops::Add;

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
    primes: Vec<u64>,
}

impl Iterator for Prime {
    type Item = u64;

    // XXX: inefficient
    fn next(&mut self) -> Option<u64> {
        let prime = self.curr;
        self.primes.push(prime);
        self.curr += 1;

        while !self.is_curr_prime() {
            self.curr += 1;
        }
        Some(prime)
    }
}

impl Prime {
    fn is_curr_prime(&self) -> bool {
        !self.primes.iter().any(|n| self.curr % n == 0)
    }
}

pub fn primes() -> Prime {
    Prime{curr: 2, primes: Vec::new()}
}

pub trait Sum<T> {
    fn sumation(self) -> T;
}

impl<T,O> Sum<O> for T where T: Iterator<Item=O>, O: Add + num::Zero {
    fn sumation(self) -> O {
        self.fold(num::zero(), |a,b| a+b)
    }
}
