use std::io::{BufRead, stdin};
use std::error;
use regex::Regex;
use lazy_static::lazy_static;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, Copy)]
enum Move {R, P, S}
#[derive(Debug, Clone, Copy)]
enum Outcome {Win, Lose, Draw}

fn parse_line(s: &str) -> Option<(Move, Move)> {
    use Move::*;
    lazy_static! {
        static ref MOVE_RE: Regex = Regex::new(r"^([ABC]) ([XYZ])$").unwrap();
    }

    let cap = MOVE_RE.captures(s)?;
    let opponent = match cap.get(1).unwrap().as_str() {
        "A" => R,
        "B" => P,
        "C" => S,
        _ => unreachable!()
    };
    let me = match cap.get(2).unwrap().as_str() {
        "X" => R,
        "Y" => P,
        "Z" => S,
        _ => unreachable!()
    };
    Some((opponent, me))
}

fn fight(opponent: Move, me: Move) -> u64 {
    use Move::*;
    let score_fight = match (opponent, me) {
        (R, P) | (P, S) | (S, R) => 6,
        (P, R) | (S, P) | (R, S) => 0,
        (R, R) | (P, P) | (S, S) => 3,
    };
    let score_shape = match me {
        R => 1, P => 2, S => 3
    };
    score_fight + score_shape
}

fn move_for_outcome(opponent: Move, outcome: Outcome) -> Move {
    use Move::*;
    use Outcome::*;
    match (opponent, outcome) {
        (S, Win) | (R, Draw) | (P, Lose) => R,
        (R, Win) | (P, Draw) | (S, Lose) => P,
        (P, Win) | (S, Draw) | (R, Lose) => S,
    }
}

pub fn aoc_2() -> Result<(u64, u64)> {
    let mut score1: u64 = 0;
    let mut score2: u64 = 0;

    let stdin = stdin();
    for line_res in stdin.lock().lines() {
        let line = line_res?;
        if let Some((opponent, me)) = parse_line(&line) {
            use Move::*;
            use Outcome::*;

            score1 += fight(opponent, me);
            let outcome = match me {R => Lose, P => Draw, S => Win};
            let me2 = move_for_outcome(opponent, outcome);
            score2 += fight(opponent, me2);
        } else {
            return Err(format!("parse error: {}", &line).into())
        }
    }

    Ok((score1, score2))
}
