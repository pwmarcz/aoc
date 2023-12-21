use color_eyre::eyre::Result;
use nom::{
    self,
    bytes::complete::tag,
    multi::{many1, separated_list1},
    sequence::terminated,
};

use crate::util::{parse_isize, parse_stdin};

fn parse_problem(s: &str) -> nom::IResult<&str, Vec<Vec<isize>>> {
    many1(terminated(
        separated_list1(tag(" "), parse_isize),
        tag("\n"),
    ))(s)
}

fn extrapolate(seq: &Vec<isize>) -> (isize, isize) {
    let mut lasts = vec![];
    let mut firsts = vec![];
    let mut cur = seq.clone();
    while !cur.iter().all(|x| *x == 0) {
        firsts.push(*cur.first().unwrap());
        lasts.push(*cur.last().unwrap());
        for i in 0..cur.len() - 1 {
            cur[i] = cur[i + 1] - cur[i];
        }
        cur.pop();
    }

    let mut first_result = 0;
    for first in firsts.iter().rev() {
        first_result = first - first_result;
    }

    let last_result = lasts.iter().sum();
    (first_result, last_result)
}

pub fn aoc_9() -> Result<(isize, isize)> {
    let sequences = parse_stdin(parse_problem)?;

    let results: Vec<(isize, isize)> = sequences.iter().map(extrapolate).collect();
    let part1 = results.iter().map(|(_first, last)| last).sum();
    let part2 = results.iter().map(|(first, _last)| first).sum();
    Ok((part1, part2))
}
