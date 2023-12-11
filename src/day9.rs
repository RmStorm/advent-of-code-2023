use itertools::Itertools;
use nom::{
    character::complete::{i32, multispace1, space1},
    multi::separated_list1,
    IResult,
};

fn parse_puzzle(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(multispace1, separated_list1(space1, i32))(input)
}

fn diff_seq(seq: &Vec<i32>) -> Vec<i32> {
    seq.iter().tuple_windows().map(|(a, b)| b - a).collect()
}

fn predict1(seq: &Vec<i32>) -> i32 {
    let dseq = diff_seq(seq);
    match dseq.iter().all(|x| x == &0) {
        true => seq[seq.len() - 1],
        false => seq[seq.len() - 1] + predict1(&dseq),
    }
}
fn predict2(seq: &Vec<i32>) -> i32 {
    let dseq = diff_seq(seq);
    match dseq.iter().all(|x| x == &0) {
        true => seq[0],
        false => seq[0] - predict2(&dseq),
    }
}

pub fn main() {
    let input = include_str!("inputs/day9.txt");
    let (_rest, seqs) = parse_puzzle(input).unwrap();
    let ans1: i32 = seqs.iter().map(|s| predict1(s)).sum();
    let ans2: i32 = seqs.iter().map(|s| predict2(s)).sum();
    println!("ans1: {}, ans2: {}", ans1, ans2);
}
