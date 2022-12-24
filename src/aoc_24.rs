use anyhow::anyhow;
use priority_queue::PriorityQueue;
use std::{
    collections::HashSet,
    io::{stdin, Read},
};

#[derive(Debug, Copy, Clone)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Blizzard {
    x: usize,
    y: usize,
    dir: Direction,
}

fn gcd(x: usize, y: usize) -> usize {
    assert!(x > 0);
    assert!(y > 0);
    let mut x = x;
    let mut y = y;
    if x < y {
        (x, y) = (y, x);
    }
    while y > 0 {
        (x, y) = (y, x % y);
    }
    x
}

impl Blizzard {
    fn pos(&self, t: usize, w: usize, h: usize) -> (usize, usize) {
        let w0 = w - 2;
        let w0_t = w0 - t % w0;
        let h0 = h - 2;
        let h0_t = h0 - t % h0;
        match self.dir {
            Direction::U => (self.x, (self.y - 1 + h0_t) % h0 + 1),
            Direction::D => (self.x, (self.y - 1 + t) % h0 + 1),
            Direction::L => ((self.x - 1 + w0_t) % w0 + 1, self.y),
            Direction::R => ((self.x - 1 + t) % w0 + 1, self.y),
        }
    }
}

struct Field {
    w: usize,
    h: usize,
    period: usize,
    blizzards: Vec<Blizzard>,
    slices: Vec<Vec<bool>>,
}

impl Field {
    fn try_from_string(s: &str) -> anyhow::Result<Self> {
        let mut lines = s.lines();
        let Some(first_line) = lines.next() else {
            return Err(anyhow!("no first line"))
        };

        let w = first_line.len();
        let mut y = 1;
        let mut blizzards = Vec::new();
        for line in lines {
            if line.len() != w {
                return Err(anyhow!("unexpected line length: {}", line.len()));
            }
            for (x, c) in line.chars().enumerate() {
                let dir = match c {
                    '^' => Some(Direction::U),
                    'v' => Some(Direction::D),
                    '<' => Some(Direction::L),
                    '>' => Some(Direction::R),
                    '#' | '.' => None,
                    _ => return Err(anyhow!("unexpected character: {}", c)),
                };
                if let Some(dir) = dir {
                    blizzards.push(Blizzard { x, y, dir });
                }
            }
            y += 1;
        }
        let h = y;

        Ok(Field {
            w,
            h,
            period: (w - 2) * (h - 2) / gcd(w - 2, h - 2),
            blizzards,
            slices: Vec::new(),
        })
    }

    fn get(&mut self, x: usize, y: usize, t: usize) -> bool {
        assert!(x <= self.w);
        assert!(y <= self.h);
        let idx = y * self.w + x;
        let slice = self.get_slice(t);
        return slice[idx];
    }

    fn get_slice(&mut self, t: usize) -> &Vec<bool> {
        let t = t % self.period;
        while self.slices.len() <= t {
            let slice = self.compute_slice(self.slices.len());
            self.slices.push(slice);
        }
        &self.slices[t]
    }

    fn compute_slice(&mut self, t: usize) -> Vec<bool> {
        let mut slice = Vec::new();
        slice.resize(self.w * self.h, true);
        for y in 0..self.h {
            slice[y * self.w] = false;
            slice[y * self.w + self.w - 1] = false;
        }
        for x in 2..(self.w - 1) {
            slice[x] = false;
            slice[self.h * self.w - x - 1] = false;
        }
        for blizzard in self.blizzards.iter() {
            let (x, y) = blizzard.pos(t, self.w, self.h);
            // dbg!(&blizzard, t, x, y);
            // println!();
            slice[y * self.w + x] = false;
        }
        slice
    }

    fn traverse(&mut self) -> Option<usize> {
        let mut queue = PriorityQueue::new();
        let mut seen = HashSet::new();
        let start = (1, 0);
        let end = (self.w - 2, self.h - 1);

        queue.push((start.0, start.1, 0), 0 as isize);
        while !queue.is_empty() {
            let ((x, y, t), _) = queue.pop().unwrap();
            // println!("pop {}, {}, {}", x, y, t);

            let seen_item = (x, y, t % self.period);
            if seen.contains(&seen_item) {
                // println!("  seen");
                continue;
            }

            seen.insert(seen_item);

            if !self.get(x, y, t) {
                // println!("  not passable");
                continue;
            }

            if (x, y) == end {
                return Some(t);
            }

            let t = t + 1;
            let p = -(t as isize);
            queue.push((x, y, t), p);
            if x > 0 {
                queue.push((x - 1, y, t), p);
            }
            if x < self.w - 1 {
                queue.push((x + 1, y, t), p);
            }
            if y > 0 {
                queue.push((x, y - 1, t), p);
            }
            if y < self.h - 1 {
                queue.push((x, y + 1, t), p);
            }
        }

        None
    }

    #[allow(dead_code)]
    fn print_at(&mut self, t: usize) {
        println!("Field at t = {}:", t);
        for y in 0..self.h {
            for x in 0..self.w {
                let can_pass = self.get(x, y, t);
                if can_pass {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
        println!();
    }
}

pub fn aoc_24() -> anyhow::Result<(usize, usize)> {
    let mut s = "".to_owned();
    stdin().read_to_string(&mut s)?;

    let mut field = Field::try_from_string(&s)?;
    // dbg!(field.period);
    // field.print_at(0);
    // field.print_at(1);
    // field.print_at(2);

    let Some(result) = field.traverse() else { return Err(anyhow!("no route found"))};
    // let result = 0;
    Ok((result, 0))
}
