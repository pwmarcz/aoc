use std::io::{stdin, Read};

use color_eyre::{eyre::eyre, Result};

#[derive(Debug)]
struct Universe {
    stars: Vec<(usize, usize)>,
    rows: Vec<bool>,
    cols: Vec<bool>,
}

impl Universe {
    fn parse(input: &str) -> Result<Self> {
        let mut stars: Vec<(usize, usize)> = vec![];
        let (mut x, mut y) = (0, 0);
        for c in input.chars() {
            if c == '\n' {
                x = 0;
                y += 1;
            } else {
                match c {
                    '.' => {}
                    '#' => stars.push((x, y)),
                    _ => return Err(eyre!("unknown character: {c:?}")),
                }
                x += 1;
            }
        }
        let w = *stars
            .iter()
            .map(|(x, _)| x)
            .max()
            .ok_or(eyre!("empty star map"))?
            + 1;
        let h = *stars
            .iter()
            .map(|(_, y)| y)
            .max()
            .ok_or(eyre!("empty star map"))?
            + 1;

        let mut rows = vec![];
        rows.resize(h, false);
        let mut cols = vec![];
        cols.resize(w, false);

        for &(x, y) in stars.iter() {
            rows[y] = true;
            cols[x] = true;
        }

        Ok(Universe { stars, rows, cols })
    }

    fn distance(&self, i: usize, j: usize, empty_size: usize) -> usize {
        let (x0, y0) = self.stars[i];
        let (x1, y1) = self.stars[j];
        let mut distance = x1.abs_diff(x0) + y1.abs_diff(y0);
        for x in x0.min(x1) + 1..x0.max(x1) {
            if !self.cols[x] {
                distance += empty_size - 1;
            }
        }
        for y in y0.min(y1) + 1..y0.max(y1) {
            if !self.rows[y] {
                distance += empty_size - 1;
            }
        }
        distance
    }

    pub fn all_distances(&self, empty_size: usize) -> usize {
        let mut result = 0;
        for i in 0..self.stars.len() - 1 {
            for j in i + 1..self.stars.len() {
                result += self.distance(i, j, empty_size);
            }
        }
        result
    }
}

pub fn aoc_11() -> Result<(usize, usize)> {
    let mut s: String = "".to_owned();
    stdin().read_to_string(&mut s)?;
    let universe = Universe::parse(&s)?;
    let part1 = universe.all_distances(2);
    let part2 = universe.all_distances(1_000_000);
    Ok((part1, part2))
}
