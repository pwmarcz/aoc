use color_eyre::Result;
use std::fmt::Debug;

use crate::util;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, terminated, tuple},
};

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn parse(s: &str) -> nom::IResult<&str, Self> {
        map(
            tuple((
                delimited(tag("Game "), util::parse_usize, tag(": ")),
                separated_list1(tag("; "), Round::parse),
            )),
            |(_, rounds)| Game { rounds },
        )(s)
    }

    fn possible(&self, red: usize, green: usize, blue: usize) -> bool {
        for round in &self.rounds {
            if !round.possible(red, green, blue) {
                return false;
            }
        }
        true
    }

    fn min_cubes(&self) -> (usize, usize, usize) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for round in &self.rounds {
            red = std::cmp::max(red, round.red);
            green = std::cmp::max(green, round.green);
            blue = std::cmp::max(blue, round.blue);
        }

        (red, green, blue)
    }
}

#[derive(Debug)]
struct Round {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Round {
    fn parse(s: &str) -> nom::IResult<&str, Self> {
        let mut round = Round {
            red: 0,
            green: 0,
            blue: 0,
        };
        let (rest, tuples) = separated_list0(
            tag(", "),
            tuple((
                util::parse_usize,
                tag(" "),
                alt((tag("red"), tag("green"), tag("blue"))),
            )),
        )(s)?;
        for &(n, _, color) in tuples.iter() {
            match color {
                "red" => round.red = n,
                "green" => round.green = n,
                "blue" => round.blue = n,
                _ => unreachable!("wrong color: {color}"),
            }
        }
        Ok((rest, round))
    }

    fn possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

pub fn aoc_2() -> Result<(usize, usize)> {
    let games = util::parse_stdin(|s| many0(terminated(Game::parse, tag("\n")))(s))?;

    let mut part1 = 0;
    let mut part2 = 0;
    for (i, game) in games.iter().enumerate() {
        if game.possible(12, 13, 14) {
            part1 += i + 1
        }
        let (red, green, blue) = game.min_cubes();
        part2 += red * green * blue;
    }

    Ok((part1, part2))
}
