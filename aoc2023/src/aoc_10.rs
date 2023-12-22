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

    fn find_cycle(&self) -> Result<Vec<(isize, isize)>> {
        let (x0, y0) = self.start()?;
        let exits = self.exits(x0, y0);
        if exits.len() != 2 {
            return Err(eyre!("expected 2 starting exits, got {exits:?}"));
        }
        let (mut x, mut y) = (x0, y0);
        let (mut dx, mut dy) = exits[0];
        let mut cycle = vec![(x0, y0)];
        loop {
            x = x + dx;
            y = y + dy;
            if (x, y) == (x0, y0) {
                return Ok(cycle);
            }
            cycle.push((x, y));
            let mut exits = self.exits(x, y);
            exits.retain(|(nx, ny)| (*nx, *ny) != (-dx, -dy));
            if exits.len() != 1 {
                return Err(eyre!("expected 1 remaining exit"));
            }
            (dx, dy) = exits[0];
        }
    }

    fn cycle_is_right_hand(cycle: &Vec<(isize, isize)>) -> bool {
        let (x0, y0) = *cycle.iter().min().unwrap();
        let i = cycle.iter().position(|&(x, y)| (x, y) == (x0, y0)).unwrap();
        let (_, y1) = cycle[(i + 1) % cycle.len()];
        y1 > y0
    }

    fn cycle_area(cycle: &Vec<(isize, isize)>) -> usize {
        let mut intersections = vec![];
        let mut cycle = cycle.clone();
        if Map::cycle_is_right_hand(&cycle) {
            cycle.reverse();
        }
        for i in 0..cycle.len() {
            let (x, y) = cycle[i];
            let (x_prev, y_prev) = cycle[(i + cycle.len() - 1) % cycle.len()];
            let (x_next, y_next) = cycle[(i + 1) % cycle.len()];

            let is_boundary = y_prev != y && y_next != y;
            let is_left_turn = !is_boundary
                && ((y_prev < y && x < x_next)
                    || (y_prev > y && x > x_next)
                    || (x_prev < x && y > y_next)
                    || (x_prev > x && y < y_next));
            if is_boundary || is_left_turn {
                intersections.push((y, x));
            }
        }
        intersections.sort();
        println!("{intersections:?}");
        let (mut x_prev, mut y_prev) = (-1, -1);
        let mut inside = false;
        let mut area = 0;
        for (y, x) in intersections {
            if inside {
                assert_eq!(y, y_prev);
                area += x - x_prev - 1;
                inside = false;
            } else {
                x_prev = x;
                y_prev = y;
                inside = true;
            }
        }
        area as usize
    }
}

pub fn aoc_10() -> Result<(usize, usize)> {
    let mut s: String = "".to_owned();
    stdin().read_to_string(&mut s)?;
    let map = Map::parse(&s)?;
    let cycle = map.find_cycle()?;
    let part1 = cycle.len() / 2;
    let part2 = Map::cycle_area(&cycle);
    Ok((part1, part2))
}
