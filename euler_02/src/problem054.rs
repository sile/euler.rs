//! [54] Poker hands
//! ----------------
//!
//! https://projecteuler.net/problem=54
//!
use std::cmp::Ordering;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq)]
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

type Value = u8; // 2..15
type Highest = Value;

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq)]
enum Rank {
    HighCard(Value),
    OnePair(Value),
    TwoPairs(Highest),
    ThreeOfAKind(Highest),
    Straight(Highest),
    Flush(Highest),
    FullHouse(Highest),
    FourOfAKind(Highest),
    StraightFlush(Highest),
    RoyalFlush,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq)]
struct Card {
    value: Value,
    suit: Suit,
}

impl Card {
    fn new(value: char, suit: char) -> Card {
        let s = match suit {
            'H' => Suit::Heart,
            'D' => Suit::Diamond,
            'C' => Suit::Club,
            'S' => Suit::Spade,
            _ => unreachable!("unknown suit: {}", suit),
        };
        let v = match value {
            '2'...'9' => value.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!("unknown value: {}", value),
        };
        Card {
            suit: s,
            value: v,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        assert_eq!(5, cards.len());
        let mut hand = Hand { cards: cards };
        hand.cards.sort();
        hand.cards.reverse();
        hand
    }
    pub fn rank(&self) -> Rank {
        let is_flush = (1..5).all(|i| self.cards[0].suit == self.cards[i].suit);
        let is_straight = (1..5).all(|i| self.cards[0].value == self.cards[i].value + i as u8);
        let highest = self.cards[0].value;
        let mut counts = [0; 15];

        for c in self.cards.iter() {
            counts[c.value as usize] += 1;
        }

        if is_flush && is_straight {
            if highest == 14 {
                return Rank::RoyalFlush;
            } else {
                return Rank::StraightFlush(highest);
            }
        }
        if let Some(i) = (2..15).find(|&i| counts[i] == 4) {
            return Rank::FourOfAKind(i as Value);
        }
        if let Some(i) = (2..15)
            .find(|&i| counts[i] == 3)
            .and_then(|x| (2..15).find(|&j| counts[j] == 2).and_then(|_| Some(x))) {
            return Rank::FullHouse(i as Value);
        }
        if is_flush {
            return Rank::Flush(highest);
        }
        if is_straight {
            return Rank::Straight(highest);
        }
        if let Some(i) = (2..15).find(|&i| counts[i] == 3) {
            return Rank::ThreeOfAKind(i as Value);
        }
        if let Some(i) = (2..15)
            .find(|&i| counts[i] == 2)
            .and_then(|x| {
                (2..15)
                    .find(|&j| x != j && counts[j] == 2)
                    .and_then(|y| { if x > y { Some(x) } else { Some(y) } })
            }) {
            return Rank::TwoPairs(i as Value);
        }
        if let Some(i) = (2..15).find(|&i| counts[i] == 2) {
            return Rank::OnePair(i as Value);
        }
        Rank::HighCard(highest)
    }
    pub fn is_winner(&self, opponent: &Hand) -> bool {
        match self.rank().cmp(&opponent.rank()) {
            Ordering::Less => false,
            Ordering::Greater => true,
            Ordering::Equal => {
                let vs0: Vec<_> = self.cards.iter().map(|c| c.value).collect();
                let vs1: Vec<_> = opponent.cards.iter().map(|c| c.value).collect();
                match vs0.cmp(&vs1) {
                    Ordering::Less => false,
                    Ordering::Greater => true,
                    Ordering::Equal => unreachable!(),
                }
            }
        }
    }
}

pub fn solve() -> u64 {
    let hands = load_hands("data/p054_poker.txt");
    hands.iter().filter(|h| h.0.is_winner(&h.1)).count() as u64
}

fn load_hands(filename: &str) -> Vec<(Hand, Hand)> {
    let mut hands = Vec::new();
    let f = File::open(Path::new(filename)).unwrap();
    let mut s = String::new();
    let mut r = BufReader::new(f);

    while let Ok(size) = r.read_line(&mut s) {
        if size == 0 {
            break;
        }
        let cs: Vec<_> = s.chars().collect();
        let pair = (Hand::new((0..05).map(|i| Card::new(cs[i * 3 + 0], cs[i * 3 + 1])).collect()),
                    Hand::new((5..10).map(|i| Card::new(cs[i * 3 + 0], cs[i * 3 + 1])).collect()));
        hands.push(pair);

        s.clear();
    }
    hands
}
