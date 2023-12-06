use std::{
    iter::{once, zip},
    ops::Range,
};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1, u64},
    combinator::map,
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}
impl Race {
    fn go(&self, holding_time: u64) -> u64 {
        (self.time - holding_time) * holding_time
    }
    fn won(&self, holding_time: u64) -> bool {
        self.go(holding_time) > self.distance
    }
}

fn parse_races_part1(input: &str) -> IResult<&str, Vec<Race>> {
    map(
        separated_pair(
            preceded(tag("Time:"), many1(preceded(space1, u64))),
            line_ending,
            preceded(tag("Distance:"), many1(preceded(space1, u64))),
        ),
        |(t, d)| {
            zip(t, d)
                .map(|(time, distance)| Race { time, distance })
                .collect()
        },
    )(input)
}
fn parse_race_part2(input: &str) -> IResult<&str, Race> {
    map(
        separated_pair(
            preceded(tag("Time:"), many1(preceded(space1, digit1))),
            line_ending,
            preceded(tag("Distance:"), many1(preceded(space1, digit1))),
        ),
        |(t, d)| Race {
            time: u64::from_str_radix(&t.join(""), 10).unwrap(),
            distance: u64::from_str_radix(&d.join(""), 10).unwrap(),
        },
    )(input)
}

pub fn main() {
    let input = include_str!("inputs/day6.txt");
    let (_rest, races) = parse_races_part1(input).unwrap();
    let ans1 = races.iter().fold(1usize, |acc, e| {
        Range {
            start: 0,
            end: e.time,
        }
        .filter(|t| e.won(*t))
        .count()
            * acc
    });
    let (_rest, race) = parse_race_part2(input).unwrap();
    let ans2 = Range {
        start: 0,
        end: race.time,
    }
    .filter(|t| race.won(*t))
    .count();
    println!("part1: {}, part2: {}", ans1, ans2);
}
