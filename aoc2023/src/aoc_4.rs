use color_eyre::eyre::Result;
use std::io::{stdin, Read};

use nom::{
    bytes::complete::tag,
    character::complete::{space1, u64, u8},
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::tuple,
};

type Number = u8;

#[derive(Debug)]
struct Card {
    winning: Vec<Number>,
    actual: Vec<Number>,
}

impl Card {
    fn parse(s: &str) -> nom::IResult<&str, Self> {
        map(
            tuple((
                tag("Card"),
                space1,
                parse_usize,
                tag(":"),
                space1,
                parse_number_list,
                tag(" |"),
                space1,
                parse_number_list,
            )),
            |(_, _, _, _, _, winning, _, _, actual)| Card { winning, actual },
        )(s)
    }

    fn score(self: &Self) -> usize {
        let mut score = 0;
        for number in &self.actual {
            if self.winning.contains(number) {
                score += 1;
            }
        }
        score
    }
}

fn parse_usize(s: &str) -> nom::IResult<&str, usize> {
    map_res(u64, usize::try_from)(s)
}

fn parse_number(s: &str) -> nom::IResult<&str, Number> {
    u8(s)
}

fn parse_number_list(s: &str) -> nom::IResult<&str, Vec<Number>> {
    separated_list1(space1, parse_number)(s)
}

pub fn aoc_4() -> Result<(usize, usize)> {
    let mut s = "".to_owned();
    stdin().read_to_string(&mut s)?;

    let cards: Vec<Card> = s
        .lines()
        .map(|line| {
            let (_rest, card) =
                all_consuming(Card::parse)(line).or_else(|err| Err(err.to_owned()))?;
            Ok(card)
        })
        .collect::<Result<Vec<Card>>>()?;

    let mut part1 = 0;
    let mut counts: Vec<usize> = cards.iter().map(|_| 1).collect();
    for (i, card) in cards.iter().enumerate() {
        let score = card.score();
        if score > 0 {
            part1 += 1 << (score - 1);
            let count = counts[i];
            for j in i + 1..=i + score {
                if j < counts.len() {
                    counts[j] += count;
                }
            }
        }
    }
    let part2 = counts.iter().sum();
    Ok((part1, part2))
}
