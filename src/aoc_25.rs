use anyhow::anyhow;
use std::io::{stdin, Read};

fn try_from_snafu(s: &str) -> anyhow::Result<isize> {
    let mut result = 0;
    for c in s.chars() {
        let digit = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => return Err(anyhow!("unrecognized digit: {}", c)),
        };
        result = result * 5 + digit;
    }
    Ok(result)
}

fn to_snafu(n: isize) -> String {
    let mut chars = vec![];
    let mut n = n;
    while n != 0 {
        let (digit, c) = match n % 5 {
            0 => (0, '0'),
            1 => (1, '1'),
            2 => (2, '2'),
            3 => (-2, '='),
            4 => (-1, '-'),
            _ => unreachable!(),
        };
        n = (n - digit) / 5;
        chars.push(c);
    }
    chars.reverse();
    String::from_iter(chars.into_iter())
}

pub fn aoc_25() -> anyhow::Result<String> {
    let mut s = "".to_owned();
    stdin().read_to_string(&mut s)?;

    let nums: Result<Vec<isize>, _> = s.lines().map(|s| try_from_snafu(s)).collect();
    let nums = nums?;

    let result = to_snafu(nums.iter().sum());

    Ok(result)
}
