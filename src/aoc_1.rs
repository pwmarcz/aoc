use std::io::BufRead;

pub fn aoc_1() -> Result<u64, std::io::Error> {
    let stdin = std::io::stdin();
    let mut max = 0;
    let mut cur = 0;
    for line in stdin.lock().lines() {
        if let Ok(num) = line?.parse::<u64>() {
            cur += num;
        } else {
            max = std::cmp::max(max, cur);
            cur = 0;
        }
    }
    Ok(std::cmp::max(max, cur))
}
