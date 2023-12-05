use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::sequence::{delimited, pair, terminated, tuple};
use nom::{bytes::complete::tag, character::complete::u32, multi::many1, IResult};

#[derive(Debug, Clone, Copy)]
pub struct Draw {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl PartialEq for Draw {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl PartialOrd for Draw {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.r <= other.r && self.g <= other.g && self.b <= other.b {
            Some(std::cmp::Ordering::Less)
        } else if self == other {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }
}

impl Draw {
    fn max(self, other: Draw) -> Draw {
        Draw {
            r: std::cmp::max(self.r, other.r),
            g: std::cmp::max(self.g, other.g),
            b: std::cmp::max(self.b, other.b),
        }
    }
    fn power(self) -> u32 {
        self.r * self.g * self.b
    }
}

impl<'a> FromIterator<(&'a str, u32)> for Draw {
    fn from_iter<I: IntoIterator<Item = (&'a str, u32)>>(iter: I) -> Self {
        let mut draw = Draw { r: 0, g: 0, b: 0 };
        for (color, value) in iter {
            match color {
                "r" => draw.r = value,
                "g" => draw.g = value,
                "b" => draw.b = value,
                _ => {}
            }
        }
        draw
    }
}

fn draw_parser(input: &str) -> IResult<&str, Draw> {
    let (remains, res) = many1(alt((
        terminated(map(u32, |i| ("r", i)), pair(tag(" red"), opt(tag(", ")))),
        terminated(map(u32, |i| ("g", i)), pair(tag(" green"), opt(tag(", ")))),
        terminated(map(u32, |i| ("b", i)), pair(tag(" blue"), opt(tag(", ")))),
    )))(input)?;
    Ok((remains, res.into_iter().collect()))
}

fn game_parser(input: &str) -> IResult<&str, (u32, Vec<Draw>)> {
    tuple((
        delimited(tag("Game "), u32, tag(": ")),
        many1(terminated(draw_parser, opt(tag("; ")))),
    ))(input.trim())
}

pub fn main() {
    let max_draw = Draw {
        r: 12,
        g: 13,
        b: 14,
    };
    let mut ans1 = 0;
    let mut ans2 = 0;
    if let Ok(lines) = crate::utils::read_lines("src/inputs/day2.txt") {
        for line in lines {
            if let Ok(chars) = line {
                let (_remaining, (index, draws)) = game_parser(&chars).unwrap();
                if draws.iter().all(|d| *d <= max_draw) {
                    ans1 += index;
                }
                ans2 += draws
                    .into_iter()
                    .fold(Draw { r: 0, g: 0, b: 0 }, |acc, d| acc.max(d))
                    .power();
            }
        }
    }
    println!("{}, {}", ans1, ans2);
}
