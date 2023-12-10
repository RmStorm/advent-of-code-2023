use core::fmt;
use std::{cmp::Ordering, iter::zip};

use nom::{
    character::complete::{anychar, line_ending, space1, u64},
    combinator::map,
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};

struct Hand {
    cards: [u8; 5],
    bid: u64,
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n\tcards: [{},{},{},{},{}]\n\tbid: {}",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4], self.bid
        )
    }
}

impl Hand {
    fn five(&self) -> bool {
        self.cards
            .iter()
            .fold(false, |a, c| self.num_j(c) == 5 || a)
    }
    fn four(&self) -> bool {
        self.cards
            .iter()
            .fold(false, |a, c| self.num_j(c) == 4 || a)
    }
    fn full_house(&self) -> bool {
        let p1 = self
            .cards
            .iter()
            .fold((false, '0' as u8), |acc, c| match self.num_j(c) == 3 {
                true => (true, *c),
                _ => acc,
            });
        let p2 = self.cards.iter().fold((false, '0' as u8), |acc, c| {
            match self.num(c) == 2 && *c != p1.1 && *c != 'J' as u8 {
                true => (true, *c),
                false => acc,
            }
        });
        p1.0 && p2.0
    }
    fn three(&self) -> bool {
        self.cards
            .iter()
            .fold(false, |a, c| self.num_j(c) == 3 || a)
    }
    fn two_pair(&self) -> bool {
        let p1 = self
            .cards
            .iter()
            .fold((false, '0' as u8), |acc, c| match self.num_j(c) == 2 {
                true => (true, *c),
                _ => acc,
            });
        let p2 = self.cards.iter().fold((false, '0' as u8), |acc, c| {
            match self.num(c) == 2 && *c != p1.1 && *c != 1 {
                true => (true, *c),
                false => acc,
            }
        });
        p1.0 && p2.0
    }
    fn pair(&self) -> bool {
        self.cards
            .iter()
            .fold(false, |a, c| self.num_j(c) == 2 || a)
    }
    fn num(&self, check_num: &u8) -> usize {
        self.cards
            .iter()
            .filter(|&card| card == check_num && *card != 1)
            .count()
    }
    fn num_j(&self, check_num: &u8) -> usize {
        self.cards
            .iter()
            .filter(|&card| card == check_num || *card == 1)
            .count()
    }
    fn tie_breaker(&self, other: &Self) -> Ordering {
        zip(self.cards, other.cards)
            .map(|(s, o)| s.cmp(&o))
            .find(|e| e != &Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }
    fn compare_type<F>(&self, other: &Self, check: F) -> Option<Ordering>
    where
        F: Fn(&Self) -> bool,
    {
        match (check(self), check(other)) {
            (true, true) => Some(self.tie_breaker(other)),
            (true, false) => Some(Ordering::Greater),
            (false, true) => Some(Ordering::Less),
            (false, false) => None,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(res) = self.compare_type(other, Hand::five) {
            Some(res)
        } else if let Some(res) = self.compare_type(other, Hand::four) {
            Some(res)
        } else if let Some(res) = self.compare_type(other, Hand::full_house) {
            Some(res)
        } else if let Some(res) = self.compare_type(other, Hand::three) {
            Some(res)
        } else if let Some(res) = self.compare_type(other, Hand::two_pair) {
            Some(res)
        } else if let Some(res) = self.compare_type(other, Hand::pair) {
            Some(res)
        } else {
            Some(self.tie_breaker(other))
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(res) = self.partial_cmp(other) {
            res
        } else {
            panic!("must be comparable")
        }
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    map(
        separated_pair(
            count(
                map(anychar, |d| match d {
                    '2'..='9' => d as u8 - '0' as u8,
                    'T' => 10,
                    'J' => 1,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("invalid"),
                }),
                5,
            ),
            space1,
            u64,
        ),
        |(t, bid)| Hand {
            cards: t.try_into().unwrap(),
            bid,
        },
    )(input)
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_hand)(input)
}

pub fn main() {
    let input = include_str!("inputs/day7.txt");
    let (_rest, mut hands) = parse_hands(input).unwrap();
    hands.sort();
    let ans = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, e)| acc + (i + 1) * e.bid as usize);
    println!("ans: {}", ans);
}
