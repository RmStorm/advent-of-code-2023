use itertools::{repeat_n, Itertools};
use nom::{character::complete::not_line_ending, combinator::map, IResult};
use std::{fmt, ops::Range};

use crate::utils::{Map, Vec2};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Galaxy,
    Distance,
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            s => panic!("Cannot cast '{}' to Space enum", s),
        }
    }
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Space::Empty => &'.',
                Space::Galaxy => &'#',
                Space::Distance => &'-',
            }
        )
    }
}

fn parse() -> Map<Space> {
    let mut area = Map {
        size: crate::utils::Vec2 { x: 0, y: 0 },
        elements: vec![],
    };
    if let Ok(lines) = crate::utils::read_lines("src/inputs/day11.txt") {
        for line in lines {
            if let Ok(chars) = line {
                let points = parse_line(&chars).unwrap().1;
                area.size.y += 1;
                area.size.x = points.len() as i64;
                area.elements.extend(points);
            }
        }
    }
    area
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Vec<Space>> {
    map(not_line_ending, |res: &str| {
        res.chars().map(|c| c.into()).collect::<Vec<Space>>()
    })(input)
}

fn find_empty_rows(space: &Map<Space>) -> Vec<i64> {
    (0..space.size.y)
        .filter(|y| {
            (0..space.size.x)
                .all(|x| space.elements[(y * space.size.x + x) as usize] != Space::Galaxy)
        })
        .collect()
}

fn find_empty_cols(space: &Map<Space>) -> Vec<i64> {
    (0..space.size.x)
        .filter(|x| {
            (0..space.size.y)
                .all(|y| space.elements[(y * space.size.x + x) as usize] != Space::Galaxy)
        })
        .collect()
}

fn expand_space(space: &mut Map<Space>) {
    for y in find_empty_rows(&space) {
        let new_row_pos = ((space.size.x * y) as usize)..((space.size.x * (y + 1)) as usize);
        let new_row = repeat_n(Space::Distance, space.size.x as usize);
        space.elements.splice(new_row_pos, new_row);
    }
    for x in find_empty_cols(&space).iter().rev() {
        for y in 0..space.size.y {
            space.elements[(y * space.size.x + x) as usize] = Space::Distance;
        }
    }
}

fn galaxy_pairs(space: &Map<Space>) -> Vec<(Vec2, Vec2)> {
    let galaxies: Vec<Vec2> = space
        .iter()
        .filter(|e| e.1 == Space::Galaxy)
        .map(|e| e.0)
        .collect();
    galaxies.into_iter().tuple_combinations().collect_vec()
}

fn range(s: i64, e: i64) -> Range<i64> {
    match s < e {
        true => s..e,
        false => e..s,
    }
}

fn find_path(space: &Map<Space>, (g1, g2): &(Vec2, Vec2)) -> Vec<Space> {
    let mut path = vec![];
    for x in range(g1.x, g2.x) {
        path.push(space.get(Vec2 { x, y: g1.y }).unwrap());
    }
    for y in range(g1.y, g2.y) {
        path.push(space.get(Vec2 { x: g2.x, y }).unwrap());
    }
    path
}

fn path_length(path: &Vec<Space>, distance: u128) -> u128 {
    path.iter().fold(0, |acc, s| {
        acc + match s {
            Space::Distance => distance,
            _ => 1,
        }
    })
}

pub fn main() {
    let mut space = parse();
    expand_space(&mut space);
    let paths = galaxy_pairs(&space)
        .iter()
        .map(|p| find_path(&space, p))
        .collect_vec();

    let ans1 = paths.iter().fold(0, |acc, p| acc + path_length(p, 2));
    let ans2 = paths.iter().fold(0, |acc, p| acc + path_length(p, 1000000));

    println!("part1: {}, part2: {}", ans1, ans2);
}
