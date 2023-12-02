use std::io::{BufRead, stdin};
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

const N_ITEMS: usize = 52;

fn get_index(b: u8) -> Option<usize> {
    match b {
        b'a' ..= b'z' => Some((b - b'a') as usize),
        b'A' ..= b'Z' => Some((b - b'A' + 26) as usize),
        _ => None
    }
}

fn score_line_2(line: &str) -> Result<usize> {
    let bytes = line.as_bytes();
    let mut left: [bool; N_ITEMS] = [false; N_ITEMS];
    let mut right: [bool; N_ITEMS] = [false; N_ITEMS];

    if bytes.len() % 2 != 0 {
        return Err("length not even".into());
    }
    for i in 0..bytes.len() {
        let b = bytes[i];
        if let Some(index) = get_index(b) {
            if i < bytes.len() / 2 {
                left[index] = true;
            } else {
                right[index] = true;
            }
        } else {
            return Err("unrecognized item".into());
        }
    }
    for item in (0..N_ITEMS).rev() {
        if left[item] && right[item] {
            return Ok(item + 1)
        }
    }

    Err("no duplicate item found".into())
}

struct Check3 {
    left: [bool; N_ITEMS],
    mid: [bool; N_ITEMS],
    right: [bool; N_ITEMS],
    score: usize,
}

impl Check3 {
    fn new() -> Self {
        Self {
            left: [false; N_ITEMS],
            mid: [false; N_ITEMS],
            right: [false; N_ITEMS],
            score: 0,
        }
    }

    fn add(&mut self, line: &str, i: usize) -> Result<()> {
        let arr = match i % 3 {
            0 => &mut self.left,
            1 => &mut self.mid,
            2 => &mut self.right,
            _ => unreachable!()
        };
        for b in line.as_bytes() {
            if let Some(index) = get_index(*b) {
                arr[index] = true;
            } else {
                return Err("unrecognized item".into());
            }
        }

        if i % 3 == 2 {
            let mut score = None;
            for item in (0..N_ITEMS).rev() {
                if self.left[item] && self.mid[item] && self.right[item] {
                    score = Some(item + 1);
                    break;
                }
            }
            if let Some(n) = score {
                self.score += n;
                self.left.fill(false);
                self.mid.fill(false);
                self.right.fill(false);
            } else {
                return Err("no triplicate item".into())
            }
        }
        Ok(())
    }
}

pub fn aoc_3() -> Result<(usize, usize)> {
    let mut score: usize = 0;
    let mut check3 = Check3::new();
    let mut i = 0;

    let stdin = stdin();
    for line_res in stdin.lock().lines() {
        let line = line_res?;
        score += score_line_2(&line)?;
        check3.add(&line, i)?;
        i += 1;
    }

    Ok((score, check3.score))
}
