use std::collections::VecDeque;
use std::io::{stdin, Read};

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u64;
use nom::combinator::{all_consuming, map, map_res, value};
use nom::multi::separated_list0;
use nom::sequence::{delimited, terminated, tuple};

use color_eyre::eyre::Result;

#[derive(Debug, Clone)]
struct Monkey {
    index: usize,
    items: VecDeque<usize>,
    operation: Operation,
    div: usize,
    if_true: usize,
    if_false: usize,
    n_inspected: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Old,
    Int(usize),
    Add(Box<Operation>, Box<Operation>),
    Mul(Box<Operation>, Box<Operation>),
}

fn parse_usize(s: &str) -> nom::IResult<&str, usize> {
    map_res(u64, usize::try_from)(s)
}

impl Monkey {
    // Monkey 1:
    //   Starting items: 54, 65, 75, 74
    //   Operation: new = old + 6
    //   Test: divisible by 19
    //     If true: throw to monkey 2
    //     If false: throw to monkey 0

    fn parse(s: &str) -> nom::IResult<&str, Self> {
        let (rest, (index, starting_items, operation, div, if_true, if_false)) = tuple((
            delimited(tag("Monkey "), parse_usize, tag(":\n")),
            delimited(
                tag("  Starting items: "),
                separated_list0(tag(", "), parse_usize),
                tag("\n"),
            ),
            delimited(tag("  Operation: new = "), Operation::parse, tag("\n")),
            delimited(tag("  Test: divisible by "), parse_usize, tag("\n")),
            delimited(tag("    If true: throw to monkey "), parse_usize, tag("\n")),
            delimited(tag("    If false: throw to monkey "), parse_usize, tag("")),
        ))(s)?;
        Ok((
            rest,
            Self {
                index,
                items: VecDeque::from(starting_items),
                operation,
                div,
                if_true,
                if_false,
                n_inspected: 0,
            },
        ))
    }

    fn parse_many(s: &str) -> nom::IResult<&str, Vec<Self>> {
        separated_list0(tag("\n\n"), Self::parse)(s)
    }

    fn receive_item(&mut self, item: usize) {
        self.items.push_back(item);
    }

    fn run(&mut self, divide: usize) -> Option<(usize, usize)> {
        if self.items.is_empty() {
            return None;
        }
        let item = self.items.pop_front().unwrap();
        println!("monkey {}: item {}", self.index, item);
        self.n_inspected += 1;
        let new_item = self.operation.run(item) / divide;
        let new_index = if new_item % self.div == 0 {
            println!(
                "  new: {}, divisible, throwing to {}",
                new_item, self.if_true
            );
            self.if_true
        } else {
            println!(
                "  new: {}, not divisible, throwing to {}",
                new_item, self.if_false
            );
            self.if_false
        };
        Some((new_item, new_index))
    }

    fn run_all(monkeys: &mut Vec<Self>, n_rounds: usize, divide: usize) -> usize {
        let modulus: usize = monkeys.iter().map(|m| m.div).product();

        for _ in 0..n_rounds {
            for i in 0..monkeys.len() {
                while let Some((new_item, new_index)) = monkeys[i].run(divide) {
                    monkeys[new_index].receive_item(new_item % modulus);
                }
            }
        }

        let score: usize = monkeys
            .iter()
            .map(|m| m.n_inspected)
            .sorted()
            .rev()
            .take(2)
            .product();

        score
    }
}

impl Operation {
    fn parse(s: &str) -> nom::IResult<&str, Self> {
        alt((
            map(
                tuple((Self::parse_simple, tag(" + "), Self::parse)),
                |(a, _, b)| Self::Add(Box::new(a), Box::new(b)),
            ),
            map(
                tuple((Self::parse_simple, tag(" * "), Self::parse)),
                |(a, _, b)| Self::Mul(Box::new(a), Box::new(b)),
            ),
            Self::parse_simple,
        ))(s)
    }

    fn parse_simple(s: &str) -> nom::IResult<&str, Self> {
        alt((value(Self::Old, tag("old")), map(parse_usize, Self::Int)))(s)
    }

    fn run(&self, old: usize) -> usize {
        match self {
            Operation::Old => old,
            Operation::Int(n) => *n,
            Operation::Add(a, b) => a.run(old) + b.run(old),
            Operation::Mul(a, b) => a.run(old) * b.run(old),
        }
    }
}

pub fn aoc_11() -> Result<(usize, usize)> {
    let mut s = "".to_owned();
    stdin().read_to_string(&mut s)?;

    let (_rest, monkeys) = all_consuming(terminated(Monkey::parse_many, tag("\n")))(&s)
        .or_else(|err| Err(err.to_owned()))?;

    let result1 = Monkey::run_all(&mut monkeys.clone(), 20, 3);
    let result2 = Monkey::run_all(&mut monkeys.clone(), 10000, 1);

    Ok((result1, result2))
}
