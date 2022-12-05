
use std::io::{stdin, Read};

use color_eyre::eyre::Result;
use color_eyre::{eyre::eyre};
use itertools::Itertools;
use regex::Regex;
use lazy_static::lazy_static;

type Item = char;

#[derive(Debug, Clone)]
struct Cargo {
    stacks: Vec<Vec<Item>>,
}

impl Cargo {
    fn try_from_lines<'a, T>(lines: T)-> Result<Self>
        where T: Iterator<Item = &'a str>
    {
        let mut cargo = Cargo{stacks: vec![]};
        for line in lines {
            cargo.add_line(&line)?;
        }
        cargo.reverse();
        Ok(cargo)
    }

    fn add_line(&mut self, line: &str) -> Result<()> {
        let chunks = line.chars().chunks(4);
        for (i, mut chunk) in chunks.into_iter().enumerate() {
            match (chunk.next(), chunk.next(), chunk.next()) {
                (Some(' '), Some(_), Some(' ')) => (),
                (Some('['), Some(item), Some(']')) => self.add_item(i, item),
                _ => return Err(eyre!("unrecognized: {:?}", line)),
            }
        }
        Ok(())
    }

    fn add_item(&mut self, i: usize, item: Item) {
        if i >= self.stacks.len() {
            self.stacks.resize(i + 1, vec![]);
        }
        self.stacks[i].push(item);
    }

    fn reverse(&mut self) {
        for stack in &mut self.stacks {
            stack.reverse()
        }
    }

    fn tops(&self) -> Result<String> {
        self.stacks.iter().map(
            |stack| stack.last().ok_or(eyre!("stack empty"))
        ).collect()
    }

    fn play_line(&mut self, line: &str, reverse: bool) -> Result<()> {
        lazy_static! {
            static ref MOVE_RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        let cap = MOVE_RE.captures(line).ok_or(eyre!("cannot parse"))?;
        let n: usize = cap[1].parse()?;
        let i: usize = cap[2].parse()?;
        let j: usize = cap[3].parse()?;
        self.play(n, i, j, reverse)?;
        Ok(())
    }

    fn play(&mut self, n: usize, i: usize, j: usize, reverse: bool) -> Result<()> {
        if i == 0 || j == 0 || i > self.stacks.len() || j > self.stacks.len() || i == j {
            return Err(eyre!("stack indexes wrong: {}, {}", i, j));
        }
        let m = self.stacks[i-1].len();
        if n > m {
            return Err(eyre!("amount too big: {}", n));
        }
        for k in 0..n {
            let idx = if reverse { m - k - 1 } else { m - n + k };
            let c = self.stacks[i-1][idx];
            self.stacks[j-1].push(c);
        }
        self.stacks[i-1].truncate(m - n);
        Ok(())
    }
}

pub fn aoc_5() -> Result<(String, String)> {
    let mut s = "".to_owned();
    stdin().read_to_string(&mut s)?;

    let (first, second) = s.split_once("\n\n").ok_or(eyre!("cannot split"))?;
    let mut cargo1 = Cargo::try_from_lines(first.lines())?;
    let mut cargo2 = cargo1.clone();

    for line in second.lines() {
        //dbg!(&cargo);
        cargo1.play_line(line, true)?;
        cargo2.play_line(line, false)?;
    }

    Ok((cargo1.tops()?, cargo2.tops()?))
}
