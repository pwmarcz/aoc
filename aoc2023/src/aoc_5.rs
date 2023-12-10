use color_eyre::eyre::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::map,
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
};

use crate::util::{self, parse_stdin};

struct RangeMap {
    ranges: Vec<(usize, usize, usize)>,
}

impl RangeMap {
    fn translate(&self, val: usize) -> usize {
        for &(dest_start, src_start, len) in &self.ranges {
            if src_start <= val && val < src_start + len {
                return dest_start + val - src_start;
            }
        }
        val
    }

    fn translate_all<'a>(maps: impl Iterator<Item = &'a Self>, val: usize) -> usize {
        let mut val = val;
        for map in maps {
            val = map.translate(val);
        }
        val
    }

    fn translate_with_rest(&self, val: usize) -> (usize, usize) {
        let mut smallest_rest = usize::MAX;
        for &(dest_start, src_start, len) in &self.ranges {
            let src_end = src_start + len;
            if src_start <= val && val < src_end {
                let rest = src_end - val - 1;
                return (dest_start + val - src_start, rest);
            } else if val < src_start {
                let rest = src_start - val - 1;
                smallest_rest = std::cmp::min(smallest_rest, rest);
            }
        }
        (val, smallest_rest)
    }

    fn translate_all_with_rest<'a>(
        maps: impl Iterator<Item = &'a Self>,
        val: usize,
    ) -> (usize, usize) {
        let mut val = val;
        let mut rest = usize::MAX;
        for map in maps {
            let (new_val, new_rest) = map.translate_with_rest(val);
            val = new_val;
            rest = std::cmp::min(rest, new_rest);
        }
        (val, rest)
    }

    fn parse(s: &str) -> nom::IResult<&str, Self> {
        map(
            preceded(
                tuple((alpha1, tag("-"), alpha1, tag("-"), alpha1, tag(" map:\n"))),
                many0(tuple((
                    terminated(util::parse_usize, tag(" ")),
                    terminated(util::parse_usize, tag(" ")),
                    terminated(util::parse_usize, tag("\n")),
                ))),
            ),
            |ranges| Self { ranges },
        )(s)
    }
}

fn parse_problem(s: &str) -> nom::IResult<&str, (Vec<usize>, Vec<RangeMap>)> {
    tuple((
        delimited(
            tag("seeds: "),
            separated_list1(tag(" "), util::parse_usize),
            tag("\n\n"),
        ),
        separated_list1(tag("\n"), RangeMap::parse),
    ))(s)
}

pub fn aoc_5() -> Result<(usize, usize)> {
    let (seeds, maps) = parse_stdin(parse_problem)?;

    let mut part1 = usize::MAX;
    for &seed in seeds.iter() {
        let val = RangeMap::translate_all(maps.iter(), seed);
        part1 = std::cmp::min(part1, val);
    }

    let mut part2 = usize::MAX;

    for (&start, &len) in seeds.iter().tuples() {
        let mut seed = start;
        while seed < start + len {
            let (val, rest) = RangeMap::translate_all_with_rest(maps.iter(), seed);
            part2 = std::cmp::min(part2, val);
            seed = seed.saturating_add(rest).saturating_add(1);
        }
    }

    Ok((part1, part2))
}
