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

#[derive(PartialOrd,PartialEq,Ord,Eq)]
struct CompositeNum {
    num: i64, // NOTE: BinaryHeapで、小さい値ほど優先度を高くしたいので、値を負数で持つようにする
    prime: u64,
}

pub struct Prime {
    curr: u64,
    composite_nums: BinaryHeap<CompositeNum>,
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let prime = self.curr;

        self.composite_nums.push(CompositeNum{num: -((prime+prime) as i64), prime: prime});
        loop {
            self.curr += 1;
            let next_composite_num = num::abs(self.composite_nums.peek().unwrap().num) as u64;
            if self.curr < next_composite_num { break }

            while self.curr == num::abs(self.composite_nums.peek().unwrap().num) as u64 {
                let mut composite_num = self.composite_nums.pop().unwrap();
                composite_num.num -= composite_num.prime as i64;
                self.composite_nums.push(composite_num);
            }
        }
        Some(prime)
    }
}

pub fn primes() -> Prime {
    Prime{curr: 2, composite_nums: BinaryHeap::new()}
}

pub trait Sum<T> {
    fn summation(self) -> T;
}

impl<T,O> Sum<O> for T where T: Iterator<Item=O>, O: Add + num::Zero {
    fn summation(self) -> O {
        self.fold(num::zero(), |a,b| a+b)
    }
}
