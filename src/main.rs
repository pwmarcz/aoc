use std::io::BufRead;


fn aoc_1() -> Result<u64, std::io::Error> {
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



fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <num>", &args[0]);
        std::process::exit(1);
    }
    match args[1].as_str() {
        "1" => println!("{}", aoc_1().unwrap()),
        _ => {
            println!("unrecognized num: {}", &args[1]);
            std::process::exit(1);
        }
    }
}
