use colored::Colorize;
use nom::{character::complete::not_line_ending, combinator::map, IResult};
use std::{collections::HashMap, fmt};

use crate::utils::Positioned;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pipe(Dir, Dir);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Pipe(Pipe),
    Start(Option<Pipe>),
    Ground,
}

#[derive(Clone, Copy)]
struct AnnotatedTile {
    tile: Tile,
    part_of_loop: bool,
    inside: bool,
}

lazy_static! {
    static ref CHAR_TO_TILE: HashMap<char, Tile> = {
        let mut m = HashMap::new();
        m.insert('|', Tile::Pipe(Pipe(Dir::South, Dir::North)));
        m.insert('-', Tile::Pipe(Pipe(Dir::West, Dir::East)));
        m.insert('L', Tile::Pipe(Pipe(Dir::North, Dir::East)));
        m.insert('J', Tile::Pipe(Pipe(Dir::North, Dir::West)));
        m.insert('7', Tile::Pipe(Pipe(Dir::South, Dir::West)));
        m.insert('F', Tile::Pipe(Pipe(Dir::South, Dir::East)));
        m.insert('S', Tile::Start(None));
        m.insert('.', Tile::Ground);
        m
    };
}

lazy_static! {
    static ref TILE_TO_CHAR: HashMap<Tile, char> =
        CHAR_TO_TILE.iter().map(|(&c, &p)| (p, c)).collect();
}

impl Dir {
    fn vec_dirs() -> [((i64, i64), Dir); 4] {
        [
            ((0, -1), Dir::South),
            ((1, 0), Dir::West),
            ((0, 1), Dir::North),
            ((-1, 0), Dir::East),
        ]
    }
    fn one80(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }
}

impl Pipe {
    fn new(dir1: Dir, dir2: Dir) -> Self {
        match TILE_TO_CHAR.contains_key(&Tile::Pipe(Pipe(dir1, dir2))) {
            true => Pipe(dir1, dir2),
            false => Pipe(dir2, dir1),
        }
    }
    fn cont(&self, from: &Dir) -> Option<Dir> {
        if self.0.one80() == *from {
            Some(self.1)
        } else if self.1.one80() == *from {
            Some(self.0)
        } else {
            None
        }
    }
    fn has(&self, dir: &Dir) -> bool {
        self.0 == *dir || self.1 == *dir
    }
}

impl AnnotatedTile {
    fn new(tile: Tile) -> AnnotatedTile {
        AnnotatedTile {
            tile,
            part_of_loop: false,
            inside: false,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        *CHAR_TO_TILE
            .get(&c)
            .unwrap_or_else(|| panic!("Could not make point from '{}'", c))
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Tile::Start(Some(_)) => &'S',
            _ => TILE_TO_CHAR.get(self).unwrap_or(&'?'),
        };
        write!(f, "{}", symbol)
    }
}

impl fmt::Debug for AnnotatedTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = format!("{:?}", self.tile);
        match (self.part_of_loop, self.inside) {
            (true, true) => write!(f, "{}", p.blue().on_red()),
            (true, false) => write!(f, "{}", p.blue()),
            (false, true) => write!(f, "{}", p.red()),
            (false, false) => write!(f, "{}", p),
        }
    }
}

fn parse() -> crate::utils::Map<Tile> {
    let mut area = crate::utils::Map {
        size: crate::utils::Vec2 { x: 0, y: 0 },
        elements: vec![],
    };
    if let Ok(lines) = crate::utils::read_lines("src/inputs/day10.txt") {
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

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Vec<Tile>> {
    map(not_line_ending, |res: &str| {
        res.chars().map(|c| c.into()).collect::<Vec<Tile>>()
    })(input)
}

pub fn main() {
    let mut area = parse();
    let mut route = vec![set_start_point(&mut area)];

    let mut next_dir = match route[0].1 {
        Tile::Start(Some(p)) => p.0,
        _ => panic!("Set the pipe in the start value to continue!"),
    };

    loop {
        let cur_element = &route[route.len() - 1];
        let mut next_pos = cur_element.0.clone();
        match next_dir {
            Dir::North => next_pos.y = next_pos.y - 1,
            Dir::East => next_pos.x = next_pos.x + 1,
            Dir::South => next_pos.y = next_pos.y + 1,
            Dir::West => next_pos.x = next_pos.x - 1,
        };
        match area.get(next_pos).unwrap() {
            Tile::Start(Some(_)) => break,
            Tile::Pipe(p) => {
                next_dir = p.cont(&next_dir).unwrap();
                route.push(Positioned(next_pos, Tile::Pipe(p)));
            }
            v => panic!("Could not complete loop '{:?}'", v),
        }
    }

    // dbg!(&route);
    let mut annotated_area = crate::utils::Map {
        size: area.size,
        elements: area
            .elements
            .iter()
            .map(|tile| AnnotatedTile::new(*tile))
            .collect(),
    };

    for tile_with_pos in &route {
        let i = tile_with_pos.0.y * area.size.x + tile_with_pos.0.x;
        annotated_area.elements[i as usize].part_of_loop = true;
    }

    let mut inside = false;
    let mut prev_corner = None;
    for e in annotated_area.elements.iter_mut() {
        if e.part_of_loop {
            if let Tile::Pipe(p) | Tile::Start(Some(p)) = e.tile {
                (prev_corner, inside) = match (p.has(&Dir::North), p.has(&Dir::South), prev_corner)
                {
                    (true, true, _) => (prev_corner, !inside),
                    (true, false, None) => (Some(Dir::North), !inside),
                    (true, false, Some(Dir::North)) => (None, !inside),
                    (true, false, Some(_)) => (None, inside),
                    (false, true, None) => (Some(Dir::South), !inside),
                    (false, true, Some(Dir::South)) => (None, !inside),
                    (false, true, Some(_)) => (None, inside),
                    (false, false, _) => (prev_corner, inside),
                };
            }
        }
        e.inside = inside;
    }
    dbg!(&annotated_area);

    let ans1 = route.len() / 2;
    let ans2 = annotated_area
        .elements
        .iter()
        .filter(|p| p.inside && !p.part_of_loop)
        .count();

    println!("part1: {}, part2: {}", ans1, ans2);
}

fn set_start_point(area: &mut crate::utils::Map<Tile>) -> Positioned<Tile> {
    let mut start = area
        .iter()
        .filter(|p| p.1 == Tile::Start(None))
        .next()
        .expect("Map must contain a start point!");

    let mut first_dir: Option<Dir> = None;
    for (diff, dir) in Dir::vec_dirs().iter() {
        let mut pos = start.0;
        pos.x += diff.0;
        pos.y += diff.1;
        if let Some(pipe) = area.get(pos) {
            let valid_dir = match pipe {
                Tile::Pipe(p) => p.has(&dir).then(|| dir.one80()),
                _ => None,
            };
            match (valid_dir, first_dir) {
                (Some(dir), None) => first_dir = Some(dir),
                (Some(dir1), Some(dir2)) => start.1 = Tile::Start(Some(Pipe::new(dir1, dir2))),
                _ => {}
            }
        }
    }
    area.elements[(start.0.y * area.size.x + start.0.x) as usize] = start.1;
    start
}
