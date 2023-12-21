use std::io::{stdin, Read};

use color_eyre::{eyre::eyre, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    Start,
}

impl Tile {
    pub fn is_connected(&self, dx: isize, dy: isize) -> bool {
        match self {
            Tile::Vertical => (dx, dy) == (0, -1) || (dx, dy) == (0, 1),
            Tile::Horizontal => (dx, dy) == (-1, 0) || (dx, dy) == (1, 0),
            Tile::BendNE => (dx, dy) == (0, -1) || (dx, dy) == (1, 0),
            Tile::BendNW => (dx, dy) == (0, -1) || (dx, dy) == (-1, 0),
            Tile::BendSW => (dx, dy) == (0, 1) || (dx, dy) == (-1, 0),
            Tile::BendSE => (dx, dy) == (0, 1) || (dx, dy) == (1, 0),
            Tile::Ground => false,
            Tile::Start => true,
        }
    }

    pub fn parse(c: char) -> Result<Self> {
        match c {
            '|' => Ok(Tile::Vertical),
            '-' => Ok(Tile::Horizontal),
            'L' => Ok(Tile::BendNE),
            'J' => Ok(Tile::BendNW),
            '7' => Ok(Tile::BendSW),
            'F' => Ok(Tile::BendSE),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            _ => Err(eyre!("unrecognized character: {c:?}")),
        }
    }
}

#[derive(Debug)]
struct Map {
    data: Vec<Tile>,
    w: isize,
    h: isize,
}

impl Map {
    fn parse(input: &str) -> Result<Self> {
        let mut w = 0;
        let mut data = vec![];
        for c in input.chars() {
            if c == '\n' {
                if w == 0 {
                    w = data.len() as isize
                };
            } else {
                let tile = Tile::parse(c)?;
                data.push(tile);
            }
        }
        if data.len() as isize % w != 0 {
            return Err(eyre!("wrong data length"));
        }
        let h = data.len() as isize / w;
        Ok(Map { data, w, h })
    }

    fn get(&self, x: isize, y: isize) -> Tile {
        assert!(0 <= x && x < self.w);
        assert!(0 <= y && y < self.h);
        self.data[(y * self.w + x) as usize]
    }

    fn start(&self) -> Result<(isize, isize)> {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.get(x, y) == Tile::Start {
                    return Ok((x, y));
                }
            }
        }
        Err(eyre!("start not found"))
    }

    fn exits(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        let mut result = vec![];
        let current = self.get(x, y);
        let mut try_exit = |dx: isize, dy: isize| {
            let other = self.get(x + dx, y + dy);
            if current.is_connected(dx, dy) && other.is_connected(-dx, -dy) {
                result.push((dx, dy));
            }
        };
        if x > 0 {
            try_exit(-1, 0);
        }
        if y > 0 {
            try_exit(0, -1);
        }
        if x < self.w - 1 {
            try_exit(1, 0);
        }
        if y < self.h - 1 {
            try_exit(0, 1);
        }
        result
    }

    fn cycle_length(&self) -> Result<usize> {
        let (x0, y0) = self.start()?;
        let exits = self.exits(x0, y0);
        if exits.len() != 2 {
            return Err(eyre!("expected 2 starting exits, got {exits:?}"));
        }
        let (mut x, mut y) = (x0, y0);
        let (mut dx, mut dy) = exits[0];
        let mut n = 0;
        loop {
            x = x + dx;
            y = y + dy;
            n += 1;
            if (x, y) == (x0, y0) {
                return Ok(n);
            }
            let mut exits = self.exits(x, y);
            exits.retain(|(nx, ny)| (*nx, *ny) != (-dx, -dy));
            if exits.len() != 1 {
                return Err(eyre!("expected 1 remaining exit"));
            }
            (dx, dy) = exits[0];
        }
    }
}

pub fn aoc_10() -> Result<(usize,)> {
    let mut s: String = "".to_owned();
    stdin().read_to_string(&mut s)?;
    let map = Map::parse(&s)?;
    let part1 = map.cycle_length()? / 2;
    Ok((part1,))
}
