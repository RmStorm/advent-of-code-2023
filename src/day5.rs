use std::{iter::once, ops::Range};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space1, u64},
    combinator::map,
    multi::{many0, many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

trait RangeExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn start_overlap(&self, other: &Self) -> bool;
    fn end_overlap(&self, other: &Self) -> bool;
}

impl RangeExt for std::ops::Range<u64> {
    fn contains_range(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn start_overlap(&self, other: &Self) -> bool {
        other.start < self.start && other.end >= self.start
    }

    fn end_overlap(&self, other: &Self) -> bool {
        other.start < self.end && other.end >= self.end
    }
}

#[derive(Debug, Clone)]
struct Mapping {
    _categories: String,
    ranges: Vec<(u64, Range<u64>)>,
}

impl Mapping {
    fn map(self: &Mapping, i: u64) -> u64 {
        for (dest, range) in self.ranges.iter() {
            if range.contains(&i) {
                return dest + i - range.start;
            }
            if i < range.start {
                return i;
            }
        }
        i
    }
    fn map_range(self: &Mapping, range_in: Range<u64>) -> Vec<Range<u64>> {
        for (dest, range) in self.ranges.iter() {
            if range.contains_range(&range_in) {
                return vec![Range {
                    start: range_in.start - range.start + dest,
                    end: range_in.end - range.start + dest,
                }];
            }
            if range.start_overlap(&range_in) {
                return vec![
                    Range {
                        start: range_in.start,
                        end: range.start,
                    },
                    Range {
                        start: *dest,
                        end: range_in.end - range.start + dest,
                    },
                ];
            }
            if range.end_overlap(&range_in) {
                return once(Range {
                    start: dest + range_in.start - range.start,
                    end: dest + range.end - range.start,
                })
                .chain(self.map_range(Range {
                    start: range.end,
                    end: range_in.end,
                }))
                .collect::<Vec<_>>();
            }
        }
        vec![range_in]
    }
    fn map_ranges(self: &Mapping, ranges_in: Vec<Range<u64>>) -> Vec<Range<u64>> {
        ranges_in
            .into_iter()
            .flat_map(|r| self.map_range(r))
            .collect()
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds:"), many1(preceded(space1, u64)))(input)
}
fn parse_point(input: &str) -> IResult<&str, (u64, Range<u64>)> {
    let nums = preceded(line_ending, separated_list1(space1, u64));
    map(nums, |nums| {
        (
            nums[0],
            Range {
                start: nums[1],
                end: nums[1] + nums[2],
            },
        )
    })(input)
}
fn parse_mapping(input: &str) -> IResult<&str, Vec<Mapping>> {
    many1(map(
        tuple((
            preceded(many0(line_ending), not_line_ending),
            many1(parse_point),
        )),
        |(c, p)| Mapping {
            _categories: c.into(),
            ranges: p,
        },
    ))(input.trim())
}

pub fn main() {
    let input = include_str!("inputs/day5.txt");
    let (rest, seeds1) = parse_seeds(input).unwrap();
    let (_rest, mut mappings) = parse_mapping(rest).unwrap();
    for m in mappings.iter_mut() {
        m.ranges.sort_by_key(|a| a.1.start)
    }

    let ans1 = seeds1
        .iter()
        .map(|s| mappings.iter().fold(*s, |acc, v| v.map(acc)))
        .min()
        .unwrap();

    let ans2 = seeds1
        .chunks(2)
        .map(|nums| {
            mappings
                .iter()
                .fold(
                    vec![Range {
                        start: nums[0],
                        end: nums[0] + nums[1],
                    }],
                    |acc, v| v.map_ranges(acc),
                )
                .iter()
                .map(|r| r.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("part1: {}, part2: {}", ans1, ans2);
}
