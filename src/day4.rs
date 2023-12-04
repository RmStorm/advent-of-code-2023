use nom::{
    bytes::complete::tag,
    character::complete::u16,
    character::complete::{space1, u32},
    combinator::map,
    multi::many1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};
use std::{collections::HashSet, fmt};

#[derive(Clone)]
struct Card {
    i: u32,
    w: HashSet<u16>,
    n: HashSet<u16>,
}
impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let index = self.i.to_string();
        write!(f, "Card {}: {:?} | {:?}", index, self.w, self.n)
    }
}

impl Card {
    fn count(&self) -> usize {
        self.w.intersection(&self.n).count()
    }
    fn points(&self) -> u64 {
        let count = self.count();
        if count == 0 {
            return 0;
        };
        2_u64.pow((count - 1) as u32)
    }
}

fn sets_parser(input: &str) -> IResult<&str, (Vec<u16>, Vec<u16>)> {
    separated_pair(
        many1(preceded(space1, u16)),
        tag(" |"),
        many1(preceded(space1, u16)),
    )(input)
}

fn card_parser(input: &str) -> IResult<&str, Card> {
    map(
        tuple((
            delimited(tuple((tag("Card"), space1)), u32, tag(":")),
            sets_parser,
        )),
        |(i, (w, n))| Card {
            i,
            w: w.into_iter().collect(),
            n: n.into_iter().collect(),
        },
    )(input.trim())
}

pub fn main() {
    let mut ans1 = 0;
    let mut card_counts = vec![];
    if let Ok(lines) = crate::utils::read_lines("inputs/day4.txt") {
        for line in lines {
            if let Ok(chars) = line {
                let (_rest, card) = card_parser(&chars).unwrap();
                ans1 += card.points();
                card_counts.push((card, 1_u64));
            }
        }
    }
    for i in 0..card_counts.len() {
        for ii in 1..(card_counts[i].0.count() + 1) {
            card_counts[i + ii].1 += card_counts[i].1;
        }
    }
    let ans2 = card_counts.iter().fold(0, |acc, card| acc + card.1);
    println!("part1: {}, part2: {}", ans1, ans2);
}
