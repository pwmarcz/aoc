use std::{
    cmp::max,
    io::{stdin, BufRead},
};

use color_eyre::{eyre::eyre, Result};

static DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_digit(line: &str, left: bool) -> Result<usize> {
    let mut found = None;
    for digit in 1..=9 {
        for pattern in [&digit.to_string(), DIGITS[digit - 1]] {
            let idx = if left {
                line.find(pattern)
            } else {
                line.rfind(pattern)
            };
            if let Some(idx) = idx {
                let key = if left {
                    line.len() - idx
                } else {
                    idx + pattern.len()
                };
                found = max(found, Some((key, digit)));
            }
        }
    }
    if let Some((_key, digit)) = found {
        Ok(digit)
    } else {
        Err(eyre!("no digits found in {line}"))
    }
}

pub fn aoc_2() -> Result<usize> {
    let stdin = stdin();
    let mut result = 0;
    for line_res in stdin.lock().lines() {
        let line: String = line_res?;
        let first = find_digit(&line, true)?;
        let last = find_digit(&line, false)?;
        eprintln!("{line} -> {first} {last}");
        result += first * 10 + last;
    }
    Ok(result)
}
