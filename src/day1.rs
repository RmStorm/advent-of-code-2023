use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take, character::complete::alpha0,
    character::complete::one_of, combinator::map, error::Error, multi::many1, sequence::preceded,
    sequence::terminated, IResult,
};
use std::io::BufRead;
use std::{fs, io};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    Ok(io::BufReader::new(fs::File::open(filename)?).lines())
}

const DIGITS: &str = "0123456789";

fn part1(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        alpha0::<&str, Error<&str>>,
        many1(terminated(
            map(one_of(DIGITS), |s: char| s.to_digit(10).unwrap()),
            alpha0,
        )),
    )(input.trim())
}

fn number_parser(input: &str) -> IResult<&str, u32> {
    alt((
        map(tag("zero"), |_| 0),
        map(tag("one"), |_| 1),
        map(tag("two"), |_| 2),
        map(tag("three"), |_| 3),
        map(tag("four"), |_| 4),
        map(tag("five"), |_| 5),
        map(tag("six"), |_| 6),
        map(tag("seven"), |_| 7),
        map(tag("eight"), |_| 8),
        map(tag("nine"), |_| 9),
        map(one_of(DIGITS), |s: char| s.to_digit(10).unwrap()),
        preceded(take(1usize), number_parser),
    ))(input)
}

fn part2(input: &str) -> IResult<&str, Vec<u32>> {
    many1(number_parser)(input.trim())
}

pub fn main() {
    let mut ans1 = 0;
    let mut ans2 = 0;
    if let Ok(lines) = read_lines("inputs/day1.txt") {
        for line in lines {
            if let Ok(chars) = line {
                let (_remaining, nums) = part1(&chars).unwrap();
                ans1 += nums.first().unwrap() * 10 + nums.last().unwrap();
                let (_remaining, nums) = part2(&chars).unwrap();
                ans2 += nums.first().unwrap() * 10 + nums.last().unwrap();
            }
        }
    }
    println!("part1: {}, part2: {}", ans1, ans2);
}
