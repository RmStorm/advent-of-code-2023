use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{map, value},
    multi::many1,
    AsChar, IResult,
};
use std::{collections::HashSet, fmt, str};

#[derive(Clone, Copy, PartialEq)]
enum Point {
    Number { i: usize, c: char, value: u32 },
    Empty,
    Symbol(char),
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Point::Number { i: _, c, value: _ } => *c,
            Point::Empty => '.',
            Point::Symbol(s) => s.as_char(),
        };
        write!(f, "{}", c)
    }
}

fn parse() -> crate::utils::Map<Point> {
    let mut schematic = crate::utils::Map {
        size: crate::utils::Vec2 { x: 0, y: 0 },
        elements: vec![],
    };
    let mut i = 0usize;
    if let Ok(lines) = crate::utils::read_lines("inputs/day3.txt") {
        for line in lines {
            if let Ok(chars) = line {
                let points = parse_line(&mut i, &chars).unwrap().1;
                schematic.size.y += 1;
                schematic.size.x = points.len() as i64;
                schematic.elements.extend(points);
            }
        }
    }
    schematic
}

fn parse_line<'a>(i: &mut usize, input: &'a str) -> IResult<&'a str, Vec<Point>> {
    map(
        many1(alt((
            map(digit1, |d: &str| {
                let value = u32::from_str_radix(d, 10).unwrap();
                *i += 1;
                d.chars()
                    .map(|c| Point::Number { i: *i, c, value })
                    .collect()
            }),
            value(vec![Point::Empty], tag(".")),
            map(take(1usize), |c: &str| {
                vec![Point::Symbol(c.chars().next().unwrap())]
            }),
        ))),
        |res| res.into_iter().flatten().collect::<Vec<Point>>(),
    )(input)
}

pub fn main() {
    let schematic = dbg!(parse());

    let mut ans1 = 0;
    let mut ans2 = 0;

    let mut added_numbers = HashSet::new();
    for el in schematic.iter() {
        if let Point::Symbol(s) = el.1 {
            let mut gear_numbers = HashSet::new();
            let mut gear_ratio = 1;
            for n in schematic.neighbor_elements(el.0) {
                if let Point::Number { i, c: _, value } = n {
                    if added_numbers.insert(i) {
                        ans1 += value;
                    }
                    if gear_numbers.insert(i) {
                        gear_ratio *= value;
                    }
                }
            }
            if s == '*' && gear_numbers.len() == 2 {
                ans2 += gear_ratio
            }
        }
    }
    println!("part1: {}, part2: {}", ans1, ans2);
}
