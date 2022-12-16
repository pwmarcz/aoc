use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::io::{stdin, Read};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::u64;
use nom::combinator::{all_consuming, map, map_res};
use nom::multi::separated_list1;
use nom::sequence::tuple;

use color_eyre::eyre::Result;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Name([char; 2]);

impl Debug for Name {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{}{}", self.0[0], self.0[1]))
    }
}

#[derive(Debug)]
struct Valve {
    name: Name,
    flow: usize,
    tunnels: Vec<Name>,
}

impl Valve {
    fn parse(s: &str) -> nom::IResult<&str, Self> {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        map(
            tuple((
                tag("Valve "),
                parse_name,
                tag(" has flow rate="),
                parse_usize,
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list1(tag(", "), parse_name),
            )),
            |(_, name, _, flow, _, tunnels)| Self {
                name,
                flow,
                tunnels,
            },
        )(s)
    }
}

fn parse_name(s: &str) -> nom::IResult<&str, Name> {
    map(
        take_while_m_n(2, 2, |c: char| 'A' <= c && c <= 'Z'),
        |s: &str| {
            let chars = [s.chars().nth(0).unwrap(), s.chars().nth(1).unwrap()];
            Name(chars)
        },
    )(s)
}

fn parse_usize(s: &str) -> nom::IResult<&str, usize> {
    map_res(u64, usize::try_from)(s)
}

const START_NAME: Name = Name(['A', 'A']);
const MINUTES: usize = 30;

#[derive(Debug)]
struct DistanceMap {
    names: Vec<Name>,
    nodes: Vec<(Name, usize)>,
    distances: HashMap<(Name, Name), usize>,
}

impl DistanceMap {
    fn new() -> Self {
        Self {
            names: vec![],
            nodes: vec![],
            distances: HashMap::new(),
        }
    }

    fn add_valve(&mut self, valve: &Valve) {
        self.names.push(valve.name);
        if valve.flow > 0 {
            self.nodes.push((valve.name, valve.flow));
        }
        self.distances.insert((valve.name, valve.name), 0);
        for tunnel in valve.tunnels.iter() {
            self.distances.insert((valve.name, *tunnel), 1);
        }
    }

    fn floyd_warshall(&mut self) {
        for c in self.names.iter() {
            for a in self.names.iter() {
                for b in self.names.iter() {
                    let ab = self.distances.get(&(*a, *b)).unwrap_or(&(usize::MAX / 2));
                    let ac = self.distances.get(&(*a, *c)).unwrap_or(&(usize::MAX / 2));
                    let cb = self.distances.get(&(*c, *b)).unwrap_or(&(usize::MAX / 2));
                    if *ab > *ac + *cb {
                        self.distances.insert((*a, *b), *ac + *cb);
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for (a, _) in self.nodes.iter() {
            for (b, _) in self.nodes.iter() {
                println!("{:?} -> {:?}: {:?}", a, b, self.distances.get(&(*a, *b)));
            }
        }
    }

    fn find_best(&self) -> usize {
        let mut best = 0;
        let mut node_set: HashSet<(Name, usize)> =
            HashSet::from_iter(self.nodes.clone().into_iter());
        let unopened = self.nodes.iter().map(|(_, flow)| flow).sum();
        self.backtrack(&mut node_set, &mut best, START_NAME, MINUTES, 0, unopened);
        best
    }

    fn backtrack(
        &self,
        node_set: &mut HashSet<(Name, usize)>,
        best: &mut usize,
        prev: Name,
        minutes_left: usize,
        total: usize,
        unopened: usize,
    ) {
        *best = (*best).max(total);

        let estimated = unopened * (minutes_left - 1);
        if total + estimated < *best {
            return;
        }

        let nodes: Vec<(Name, usize)> = node_set.iter().cloned().collect();
        for (name, flow) in nodes.iter() {
            let Some(dist) = self.distances.get(&(prev, *name)) else { continue };
            if minutes_left > dist + 1 {
                let minutes_left = minutes_left - dist - 1;
                let total = total + flow * minutes_left;
                let prev = *name;
                let unopened = unopened - flow;
                node_set.remove(&(*name, *flow));
                self.backtrack(node_set, best, prev, minutes_left, total, unopened);
                node_set.insert((*name, *flow));
            }
        }
    }
}

pub fn aoc_16() -> Result<usize> {
    let mut s = "".to_owned();
    stdin().read_to_string(&mut s)?;

    let mut map = DistanceMap::new();

    for line in s.lines() {
        let (_rest, valve) =
            all_consuming(Valve::parse)(line).or_else(|err| Err(err.to_owned()))?;

        map.add_valve(&valve);
    }
    map.floyd_warshall();
    //map.dump();
    let result = map.find_best();
    Ok(result)
}
