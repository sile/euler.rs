extern crate num;

use std::ops::Add;
use std::collections::BinaryHeap;

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

use std::cmp::PartialOrd;
use std::cmp::Ordering;

#[derive(PartialEq,Eq,Ord)]
struct Num {
    val: u64,
    prime: u64,
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Num) -> Option<Ordering> {
        if      self.val < other.val { return Some(Ordering::Greater) }
        else if self.val > other.val { return Some(Ordering::Less) }
        else                         { return Some(Ordering::Equal) }
    }
}

pub struct Prime {
    nums: BinaryHeap<Num>,
}

impl Prime {
    pub fn push_next_prime(&mut self, prime: u64) {
        self.nums.push(Num{val: prime, prime: prime});
    }
    pub fn pop_next_prime(&mut self) -> u64 {
        self.pop_and_repush().val
    }
    pub fn is_prime(&mut self, n: u64) -> bool {
        if n < self.nums.peek().unwrap().val {
            true
        } else {
            while n == self.nums.peek().unwrap().val { self.pop_and_repush(); };
            false
        }
    }
    fn pop_and_repush(&mut self) -> Num {
        let num = self.nums.pop().unwrap();
        self.nums.push(Num{val: num.val + num.prime, prime: num.prime});
        num
    }
}

impl Iterator for Prime {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let prime = self.pop_next_prime();
        let next = (prime+1..).find(|n| self.is_prime(*n) ).unwrap();
        self.push_next_prime(next);
        Some(prime)
    }
}

pub fn primes() -> Prime {
    let mut p = Prime{nums: BinaryHeap::new()};
    p.push_next_prime(2);
    p
}

pub trait Sum<T> {
    fn summation(self) -> T;
}

impl<T,O> Sum<O> for T where T: Iterator<Item=O>, O: Add + num::Zero {
    fn summation(self) -> O {
        self.fold(num::zero(), |a,b| a+b)
    }
}
