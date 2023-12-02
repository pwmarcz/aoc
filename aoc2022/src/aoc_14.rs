use std::cmp::max;
use std::io::{stdin, Read};

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::u64;
use nom::combinator::{all_consuming, map_res};
use nom::multi::{many0, separated_list0};
use nom::sequence::{separated_pair, terminated};

use color_eyre::eyre::Result;

type Line = (usize, usize, usize, usize);

#[derive(PartialEq, Eq)]
enum Outcome {
    Rest,
    Fall,
    Blocked,
}

const START_X: usize = 500;
const START_Y: usize = 0;

#[derive(Debug)]
struct Sandbox {
    x0: usize,
    y0: usize,
    w: usize,
    h: usize,
    grid: Vec<bool>,
}

fn delta<T>(a: T, b: T) -> isize
where
    T: Ord,
{
    if a < b {
        1
    } else if a > b {
        -1
    } else {
        0
    }
}

impl Sandbox {
    fn new(x0: usize, y0: usize, w: usize, h: usize) -> Self {
        let mut grid = Vec::new();
        grid.resize(w * h, false);
        Sandbox { x0, y0, w, h, grid }
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x >= self.x0 && x < self.x0 + self.w && y >= self.y0 && y < self.y0 + self.h
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if !self.in_bounds(x, y) {
            return false;
        }
        self.grid[(y - self.y0) * self.w + x - self.x0]
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        self.grid[(y - self.y0) * self.w + x - self.x0] = val
    }

    fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let dx = delta(x1, x2);
        let dy = delta(y1, y2);
        let mut x = x1;
        let mut y = y1;
        while x != x2 || y != y2 {
            self.set(x, y, true);
            x = ((x as isize) + dx) as usize;
            y = ((y as isize) + dy) as usize;
        }
        self.set(x, y, true);
    }

    fn from_lines(lines: &Vec<Line>, bottom: bool) -> Self {
        let x0 = 0;
        let xmax = 1000;
        let y0 = 0;
        let ymax = lines
            .into_iter()
            .map(|(_, y1, _, y2)| max(*y1, *y2))
            .max()
            .unwrap();
        let mut sandbox = Self::new(x0, y0, xmax - x0 + 1, ymax - y0 + 3);
        for (x1, y1, x2, y2) in lines {
            sandbox.draw_line(*x1, *y1, *x2, *y2);
        }
        if bottom {
            sandbox.draw_line(x0, ymax + 2, xmax, ymax + 2)
        }
        sandbox
    }

    fn try_grain(&mut self) -> Outcome {
        let mut x = START_X;
        let mut y = START_Y;
        if self.get(x, y) {
            return Outcome::Blocked;
        }
        loop {
            if !self.get(x, y + 1) {
                y += 1;
            } else if !self.get(x - 1, y + 1) {
                x -= 1;
                y += 1;
            } else if !self.get(x + 1, y + 1) {
                x += 1;
                y += 1;
            } else {
                self.set(x, y, true);
                return Outcome::Rest;
            }
            if !self.in_bounds(x, y) {
                return Outcome::Fall;
            }
        }
    }

    fn count_grains_until(&mut self, outcome: Outcome) -> usize {
        let mut i = 0;
        loop {
            if self.try_grain() == outcome {
                return i;
            }
            i += 1;
        }
    }
}

fn parse_file(s: &str) -> nom::IResult<&str, Vec<Line>> {
    let (rest, all_lines) = many0(terminated(parse_lines, tag("\n")))(s)?;
    let lines: Vec<Line> = all_lines.into_iter().flatten().collect();
    Ok((rest, lines))
}

fn parse_lines(s: &str) -> nom::IResult<&str, Vec<Line>> {
    let (rest, pairs) = separated_list0(
        tag(" -> "),
        separated_pair(parse_usize, tag(","), parse_usize),
    )(s)?;

    let lines: Vec<Line> = pairs
        .into_iter()
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| (x1, y1, x2, y2))
        .collect();

    Ok((rest, lines))
}

fn parse_usize(s: &str) -> nom::IResult<&str, usize> {
    map_res(u64, usize::try_from)(s)
}

pub fn aoc_14() -> Result<(usize, usize)> {
    let mut s = "".to_owned();
    stdin().read_to_string(&mut s)?;

    let (_rest, lines) = all_consuming(parse_file)(&s).or_else(|err| Err(err.to_owned()))?;

    let mut sandbox1 = Sandbox::from_lines(&lines, false);
    let result1 = sandbox1.count_grains_until(Outcome::Fall);

    let mut sandbox2 = Sandbox::from_lines(&lines, true);
    let result2 = sandbox2.count_grains_until(Outcome::Blocked);

    Ok((result1, result2))
}
