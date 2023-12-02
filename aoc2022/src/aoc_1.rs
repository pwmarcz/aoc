use std::io::BufRead;

pub fn aoc_1() -> Result<(u64, u64), std::io::Error> {
    let stdin = std::io::stdin();
    let mut best: [u64; 4] = [0; 4];
    let mut max = 0;
    let mut cur = 0;
    for line in stdin.lock().lines() {
        if let Ok(num) = line?.parse::<u64>() {
            cur += num;
        } else {
            max = std::cmp::max(max, cur);
            best[0] = cur;
            best.sort();
            cur = 0;
        }
    }
    max = std::cmp::max(max, cur);
    best[0] = cur;
    best.sort();
    Ok((max, best[1] + best[2] + best[3]))
}
