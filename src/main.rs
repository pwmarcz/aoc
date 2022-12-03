mod aoc_1;
mod aoc_2;

use aoc_1::aoc_1;
use aoc_2::aoc_2;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <num>", &args[0]);
        std::process::exit(1);
    }
    match args[1].as_str() {
        "1" => println!("{}", aoc_1().unwrap()),
        "2" => println!("{:?}", aoc_2().unwrap()),
        _ => {
            println!("unrecognized num: {}", &args[1]);
            std::process::exit(1);
        }
    }
}
