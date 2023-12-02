use std::io::{stdin, Read};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::combinator::{all_consuming, map, map_res};
use nom::multi::many0;
use nom::sequence::{preceded, terminated};

use color_eyre::eyre::Result;

fn parse_program(s: &str) -> nom::IResult<&str, Vec<Instr>> {
    many0(terminated(parse_instr, tag("\n")))(s)
}

fn parse_instr(s: &str) -> nom::IResult<&str, Instr> {
    alt((
        map(tag("noop"), |_| Instr::Noop),
        map(
            map_res(preceded(tag("addx "), i64), |n: i64| n.try_into()),
            Instr::AddX,
        ),
    ))(s)
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    Noop,
    AddX(isize),
}

struct Sim<'a> {
    program: &'a Vec<Instr>,
    x: isize,
    pc: usize,
    pc_cycle: usize,
    cycle: usize,
}

impl<'a> From<&'a Vec<Instr>> for Sim<'a> {
    fn from(program: &'a Vec<Instr>) -> Self {
        Self {
            program,
            x: 1,
            pc: 0,
            pc_cycle: 0,
            cycle: 1,
        }
    }
}

impl<'a> Sim<'a> {
    fn finished(&self) -> bool {
        self.pc >= self.program.len()
    }

    fn step(&mut self) {
        assert!(!self.finished());
        let instr = self.program[self.pc];
        let next = match (instr, self.pc_cycle) {
            (Instr::Noop, 0) => true,
            (Instr::Noop, _) => unreachable!(),
            (Instr::AddX(_), 0) => false,
            (Instr::AddX(n), 1) => {
                self.x += n;
                true
            }
            (Instr::AddX(_), _) => unreachable!(),
        };
        if next {
            self.pc += 1;
            self.pc_cycle = 0;
        } else {
            self.pc_cycle += 1;
        }
        self.cycle += 1;
    }

    pub fn draw_pixel(&mut self) {
        if self.cycle <= 240 {
            let col = ((self.cycle - 1) % 40) as isize;
            let lit = self.x - 1 <= col && col <= self.x + 1;
            print!("{}", if lit { '#' } else { '.' });
            if col == 39 {
                println!();
            }
        }
    }

    pub fn run_and_measure(mut self) -> isize {
        let mut result = 0;
        while !self.finished() {
            self.draw_pixel();
            self.step();
            // println!("during cycle {} signal is {}", self.cycle, self.x);
            if self.cycle >= 20 && (self.cycle - 20) % 40 == 0 {
                result += self.cycle as isize * self.x;
            }
        }
        result
    }
}

pub fn aoc_10() -> Result<isize> {
    let mut s = "".to_owned();
    stdin().read_to_string(&mut s)?;

    let (_rest, program) = all_consuming(parse_program)(&s).or_else(|err| Err(err.to_owned()))?;

    let sim = Sim::from(&program);
    let result = sim.run_and_measure();

    Ok(result)
}
