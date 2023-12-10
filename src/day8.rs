use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0, multispace1},
    combinator::map,
    multi::fold_many1,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

fn parse_map_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            pair(tag(")"), multispace0),
        ),
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    fold_many1(parse_map_line, HashMap::new, |mut map, (key, value)| {
        map.insert(key, value);
        map
    })(input)
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Dir>> {
    map(alpha1, |i: &str| {
        i.chars()
            .map(|d| match d {
                'L' => Dir::Left,
                'R' => Dir::Right,
                s => {
                    dbg!(s);
                    panic!("invalid")
                }
            })
            .collect()
    })(input)
}

fn parse_puzzle(input: &str) -> IResult<&str, (Vec<Dir>, HashMap<&str, (&str, &str)>)> {
    separated_pair(parse_instructions, multispace1, parse_map)(input)
}

pub fn main() {
    let input = include_str!("inputs/day8.txt");
    let (_rest, (dirs, map)) = parse_puzzle(input).unwrap();
    let mut ans1 = 0;
    let mut cur = "AAA";
    for d in dirs.iter().cycle() {
        cur = match d {
            Dir::Left => map.get(&cur).unwrap().0,
            Dir::Right => map.get(&cur).unwrap().1,
        };
        ans1 += 1;
        if cur == "ZZZ" {
            break;
        }
    }

    let starting_points: Vec<&&str> = map
        .keys()
        .filter(|k| k.chars().nth(2) == Some('A'))
        .collect();
    let mut periods = vec![];
    for sp in starting_points {
        let mut cur = *sp;
        let mut steps = 0;

        for d in dirs.iter().cycle() {
            cur = match d {
                Dir::Left => map.get(&cur).unwrap().0,
                Dir::Right => map.get(&cur).unwrap().1,
            };
            steps += 1;
            if cur.chars().nth(2).unwrap() == 'Z' {
                periods.push(steps as u64);
                break;
            }
        }
    }
    let ans2 = periods
        .into_iter()
        .reduce(|acc, x| num::integer::lcm(acc, x))
        .unwrap();
    println!("ans1: {}, ans2: {}", ans1, ans2);
}
