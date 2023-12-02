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

struct Backtrack<'a, const N: usize> {
    map: &'a DistanceMap,
    node_set: HashSet<(Name, usize)>,
    best: usize,
}

struct BacktrackState<const N: usize> {
    current: [Name; N],
    next_move: [usize; N],
    minutes_left: usize,
    total: usize,
    unopened: usize,
}

impl<'a, const N: usize> Backtrack<'a, N> {
    fn new(map: &'a DistanceMap) -> Self {
        Self {
            map,
            node_set: HashSet::from_iter(map.nodes.clone().into_iter()),
            best: 0,
        }
    }

    fn backtrack(&mut self, minutes: usize) -> usize {
        let state = BacktrackState {
            current: [START_NAME; N],
            next_move: [minutes; N],
            minutes_left: minutes,
            total: 0,
            unopened: self.map.nodes.iter().map(|(_, flow)| flow).sum(),
        };
        self.move_step(state, 0);
        self.best
    }

    fn get_distance(&self, a: Name, b: Name) -> Option<usize> {
        self.map.distances.get(&(a, b)).copied()
    }

    fn move_step(&mut self, state: BacktrackState<N>, i: usize) {
        if i == N {
            return self.wait_step(state);
        }
        if state.next_move[i] < state.minutes_left {
            return self.move_step(state, i + 1);
        }
        let prev = state.current[i];
        let nodes: Vec<(Name, usize)> = self.node_set.iter().cloned().collect();
        for node @ (name, flow) in nodes.iter() {
            self.node_set.remove(node);

            if let Some(dist) = self.get_distance(prev, *name) {
                if dist + 1 < state.minutes_left {
                    let mut current = state.current.clone();
                    current[i] = *name;
                    let mut next_move = state.next_move.clone();
                    next_move[i] = state.minutes_left - dist - 1;
                    let state = BacktrackState {
                        current,
                        next_move,
                        total: state.total + flow * (state.minutes_left - dist - 1),
                        unopened: state.unopened - flow,
                        ..state
                    };
                    self.move_step(state, i + 1);
                }
            }

            self.node_set.insert(*node);
        }
    }

    fn wait_step(&mut self, state: BacktrackState<N>) {
        self.best = self.best.max(state.total);

        if self.node_set.is_empty() {
            return;
        }
        let minutes_left = *state.next_move.iter().max().unwrap();
        assert!(minutes_left < state.minutes_left);

        let estimated = state.unopened * (minutes_left - 1);
        if state.total + estimated <= self.best {
            return;
        }

        self.move_step(
            BacktrackState {
                minutes_left,
                ..state
            },
            0,
        );
    }
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
        let mut state = Backtrack::<1>::new(self);
        state.backtrack(MINUTES)
    }

    fn find_best_2(&self) -> usize {
        let mut state = Backtrack::<2>::new(self);
        state.backtrack(MINUTES - 4)
    }
}

pub fn aoc_16() -> Result<(usize, usize)> {
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
    let result2 = map.find_best_2();

    Ok((result, result2))
}
