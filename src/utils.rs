use std::io::BufRead;
use std::{fmt, fs, io};

#[allow(dead_code)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    Ok(io::BufReader::new(fs::File::open(filename)?).lines())
}

// Map type from https://fasterthanli.me/series/advent-of-code-2020/part-11
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

pub struct Map<T> {
    pub size: Vec2,
    pub elements: Vec<T>,
}

impl<T> Map<T> {
    fn index(&self, pos: Vec2) -> Option<usize> {
        if (0..self.size.x).contains(&pos.x) && (0..self.size.y).contains(&pos.y) {
            Some((pos.x + pos.y * self.size.x) as _)
        } else {
            None
        }
    }
}

impl<T> Map<T>
where
    T: Copy,
{
    fn get(&self, pos: Vec2) -> Option<T> {
        self.index(pos).map(|index| self.elements[index])
    }
}

impl<T> fmt::Debug for Map<T>
where
    T: fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{:?}", self.get(Vec2 { x, y }).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Map<T> {
    #[allow(dead_code)]
    fn neighbor_positions(&self, pos: Vec2) -> impl Iterator<Item = Vec2> {
        (-1..=1)
            .map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .flatten()
            .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
            .map(move |(dx, dy)| Vec2 {
                x: pos.x + dx,
                y: pos.y + dy,
            })
    }
}

#[derive(Debug)]
pub struct Positioned<T>(pub Vec2, pub T);

impl<T> Map<T>
where
    T: Copy,
{
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = Positioned<T>> + '_ {
        (0..self.size.y)
            .map(move |y| {
                (0..self.size.x).map(move |x| {
                    let pos = Vec2 { x, y };
                    Positioned(pos, self.get(pos).unwrap())
                })
            })
            .flatten()
    }
    #[allow(dead_code)]
    pub fn neighbor_elements(&self, pos: Vec2) -> impl Iterator<Item = T> + '_ {
        self.neighbor_positions(pos)
            .filter_map(move |pos| self.get(pos))
    }
}
