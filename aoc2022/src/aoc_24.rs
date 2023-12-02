use anyhow::anyhow;
use std::{
    collections::{HashSet, VecDeque},
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
    template: Vec<bool>,
    slices: Vec<Vec<bool>>,
}

impl Field {
    fn try_from_string(s: &str) -> anyhow::Result<Self> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() == 0 {
            return Err(anyhow!("no first line"));
        }

        let w = lines[0].len();
        let h = lines.len();
        let mut blizzards = Vec::new();
        let mut template = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            if line.len() != w {
                return Err(anyhow!("unexpected line length: {}", line.len()));
            }
            for (x, c) in line.chars().enumerate() {
                if c == '\n' {
                    break;
                }
                template.push(c != '#');
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
        }

        assert!(template.len() == w * h);

        Ok(Field {
            w,
            h,
            period: (w - 2) * (h - 2) / gcd(w - 2, h - 2),
            blizzards,
            slices: Vec::new(),
            template,
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
        let mut slice = self.template.clone();
        for blizzard in self.blizzards.iter() {
            let (x, y) = blizzard.pos(t, self.w, self.h);
            // dbg!(&blizzard, t, x, y);
            // println!();
            slice[y * self.w + x] = false;
        }
        slice
    }

    fn start(&self) -> (usize, usize) {
        (1, 0)
    }

    fn end(&self) -> (usize, usize) {
        (self.w - 2, self.h - 1)
    }

    fn traverse(
        &mut self,
        t0: usize,
        start: (usize, usize),
        end: (usize, usize),
    ) -> anyhow::Result<usize> {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        queue.push_back((start.0, start.1, t0));
        while !queue.is_empty() {
            let (x, y, t) = queue.pop_front().unwrap();
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
                return Ok(t);
            }

            queue.push_back((x, y, t + 1));
            if x > 0 {
                queue.push_back((x - 1, y, t + 1));
            }
            if x < self.w - 1 {
                queue.push_back((x + 1, y, t + 1));
            }
            if y > 0 {
                queue.push_back((x, y - 1, t + 1));
            }
            if y < self.h - 1 {
                queue.push_back((x, y + 1, t + 1));
            }
        }

        Err(anyhow!(
            "no route found from {:?} to {:?} at {}",
            start,
            end,
            t0
        ))
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

    let t1 = field.traverse(0, field.start(), field.end())?;
    let t2 = field.traverse(t1, field.end(), field.start())?;
    let t3 = field.traverse(t2, field.start(), field.end())?;
    // let result = 0;
    Ok((t1, t3))
}
