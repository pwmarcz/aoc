use color_eyre::eyre::Result;
use itertools::Itertools;

fn ways_to_beat(time: usize, distance: usize) -> usize {
    // 0 <= x <= t
    // (t - x) * x > d
    // -x^2 + tx - d - 1 >= 0

    let delta = time * time - 4 * (distance + 1);
    let sqrt_delta = (delta as f64).sqrt();
    let x0f = (time as f64 - sqrt_delta) / 2.0;
    let x1f: f64 = (time as f64 + sqrt_delta) / 2.0;
    let x0 = x0f.ceil() as usize;
    let x1 = x1f.floor() as usize;
    x1 - x0 + 1
}

fn part1(games: &Vec<(usize, usize)>) -> usize {
    games
        .iter()
        .map(|(time, distance)| ways_to_beat(*time, *distance))
        .product()
}

fn part2(games: &Vec<(usize, usize)>) -> usize {
    let time_str = games
        .iter()
        .map(|(time, _distance)| time.to_string())
        .join("");
    let distance_str = games
        .iter()
        .map(|(_time, distance)| distance.to_string())
        .join("");
    let time = time_str.parse().unwrap();
    let distance = distance_str.parse().unwrap();
    ways_to_beat(time, distance)
}

pub fn aoc_6() -> Result<(usize, usize, usize, usize)> {
    let demo_input = vec![(7, 9), (15, 40), (30, 200)];
    let real_input = vec![(47, 282), (70, 1079), (75, 1147), (66, 1062)];

    Ok((
        part1(&demo_input),
        part1(&real_input),
        part2(&demo_input),
        part2(&real_input),
    ))
}
