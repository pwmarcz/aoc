use std::{
    collections::HashMap,
    io::{stdin, Read},
};

use color_eyre::eyre::Result;
use itertools::Itertools;

#[derive(Clone)]
struct Rock {
    lines: [u8; 4],
}

#[derive(Clone, Copy, Debug)]
enum Shift {
    Left,
    Right,
}

impl Rock {
    fn shift(&mut self, shift: Shift, lines: &[u8; 4]) -> bool {
        match shift {
            Shift::Left => {
                for i in 0..4 {
                    if self.lines[i] & 0b0100_0000 != 0 || (self.lines[i] << 1) & lines[i] != 0 {
                        return false;
                    }
                }
                for x in self.lines.iter_mut() {
                    *x <<= 1;
                }
                return true;
            }
            Shift::Right => {
                for i in 0..4 {
                    if self.lines[i] & 0b1 != 0 || (self.lines[i] >> 1) & lines[i] != 0 {
                        return false;
                    }
                }
                for x in self.lines.iter_mut() {
                    *x >>= 1;
                }
                return true;
            }
        }
    }

    fn intersect(&self, lines: &[u8; 4]) -> bool {
        (0..4).any(|i| self.lines[i] & lines[i] != 0)
    }

    fn apply(&self, lines: &mut [u8; 4]) {
        for i in 0..4 {
            assert!(self.lines[i] & lines[i] == 0);
            lines[i] |= self.lines[i];
        }
    }
}

const ROCKS: [Rock; 5] = [
    Rock {
        lines: [0b0001_1110, 0, 0, 0],
    },
    Rock {
        lines: [0b0000_1000, 0b0001_1100, 0b0000_1000, 0],
    },
    Rock {
        lines: [0b0001_1100, 0b0000_0100, 0b0000_0100, 0],
    },
    Rock {
        lines: [0b0001_0000, 0b0001_0000, 0b0001_0000, 0b0001_0000],
    },
    Rock {
        lines: [0b0001_1000, 0b0001_1000, 0, 0],
    },
];

const SNAPSHOT_SIZE: usize = 32;

#[derive(PartialEq, Eq, Hash)]
struct Snapshot {
    n_turns_mod: usize,
    n_rocks_mod: usize,
    last_rows: [u8; SNAPSHOT_SIZE],
}

#[derive(Copy, Clone, Debug)]
struct Period {
    n_rocks: usize,
    n_rocks_delta: usize,
    top_delta: usize,
}

struct Well {
    lines: Vec<u8>,
    shifts: Vec<Shift>,
    rock: Rock,
    rock_pos: usize,
    n_turns: usize,
    n_rocks: usize,

    snapshots: HashMap<Snapshot, (usize, usize)>,
    period: Option<Period>,
}

impl Well {
    fn new(shifts: Vec<Shift>) -> Self {
        Self {
            lines: vec![0],
            shifts,
            rock: ROCKS[0].clone(),
            rock_pos: 3,
            n_turns: 0,
            n_rocks: 0,
            snapshots: HashMap::new(),
            period: None,
        }
    }

    fn lines4(lines: &mut Vec<u8>, i: usize) -> &mut [u8; 4] {
        if lines.len() < i + 4 {
            lines.resize(i + 4, 0);
        }
        let slice = &mut lines[i..i + 4];
        slice.try_into().unwrap()
    }

    fn tick(&mut self) {
        let shift = self.shifts[self.n_turns % self.shifts.len()];
        let lines = Self::lines4(&mut self.lines, self.rock_pos);
        let _success = self.rock.shift(shift, lines);
        // println!(
        //     "shift {:?}{}",
        //     shift,
        //     if success { "" } else { " (nothing happens)" }
        // );

        if self.rock_pos > 0
            && !self
                .rock
                .intersect(Self::lines4(&mut self.lines, self.rock_pos - 1))
        {
            // println!("down");
            self.rock_pos -= 1;
        } else {
            self.snapshot();

            // println!("rest");
            let lines = Self::lines4(&mut self.lines, self.rock_pos);
            self.rock.apply(lines);
            self.new_rock();
        }
        self.n_turns += 1;
    }

    fn snapshot(&mut self) {
        if self.period.is_some() {
            return;
        }
        if self.top() < SNAPSHOT_SIZE {
            return;
        }
        let snapshot = Snapshot {
            n_turns_mod: self.n_turns % ROCKS.len(),
            n_rocks_mod: self.n_rocks % ROCKS.len(),
            last_rows: self.lines[self.top() - SNAPSHOT_SIZE..self.top()]
                .try_into()
                .unwrap(),
        };
        if let Some((n_rocks, top)) = self.snapshots.get(&snapshot).cloned() {
            self.period = Some(Period {
                n_rocks,
                n_rocks_delta: self.n_rocks - n_rocks,
                top_delta: self.top() - top,
            })
        } else {
            self.snapshots.insert(snapshot, (self.n_rocks, self.top()));
        }
    }

    fn top(&self) -> usize {
        for i in (0..self.lines.len()).rev() {
            if self.lines[i] != 0 {
                return i + 1;
            }
        }
        0
    }

    fn new_rock(&mut self) {
        self.n_rocks += 1;
        self.rock = ROCKS[self.n_rocks % ROCKS.len()].clone();
        self.rock_pos = self.top() + 3;
        // println!("rock {}, top {}", self.n_rocks, self.top());
    }

    fn simulate(&mut self, n_rocks: usize) {
        while self.n_rocks < n_rocks {
            self.tick();
        }
    }

    fn find_period(&mut self) -> Period {
        while self.period.is_none() {
            self.tick();
        }
        self.period.to_owned().unwrap()
    }

    #[allow(dead_code)]
    fn dump(&self, n: usize) {
        let top = self.top();
        for i in 0..n {
            for j in (0..7).rev() {
                let cell = self.lines[top - i - 1] & (1 << j) != 0;
                print!("{}", if cell { '#' } else { '.' });
            }
            println!();
        }
    }
}

pub fn aoc_17() -> Result<(usize, usize)> {
    let mut s = "".to_owned();
    stdin().read_to_string(&mut s)?;

    let shifts = s
        .chars()
        .into_iter()
        .filter_map(|c| match c {
            '<' => Some(Shift::Left),
            '>' => Some(Shift::Right),
            _ => None,
        })
        .collect_vec();

    dbg!(shifts.len());

    let mut well = Well::new(shifts.clone());
    well.simulate(2022);
    let result = well.top();

    let period = well.find_period();
    dbg!(period);

    let goal = 1000000000000;

    let mut well = Well::new(shifts.clone());
    well.simulate(period.n_rocks + (goal - period.n_rocks) % period.n_rocks_delta);
    let result2 = well.top() + (goal - period.n_rocks) / period.n_rocks_delta * period.top_delta;

    // let mut well = Well::new(shifts.clone());
    // well.simulate_until_period();
    // dbg!(n0, t0, n1, t1);

    Ok((result, result2))
}
