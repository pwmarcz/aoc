mod aoc_1;
mod aoc_2;
mod aoc_3;
mod aoc_5;

mod aoc_10;
mod aoc_11;
mod aoc_14;
mod aoc_16;
mod aoc_17;

use aoc_1::aoc_1;
use aoc_2::aoc_2;
use aoc_3::aoc_3;
use aoc_5::aoc_5;

use aoc_10::aoc_10;
use aoc_11::aoc_11;
use aoc_14::aoc_14;
use aoc_16::aoc_16;
use aoc_17::aoc_17;

fn main() {
    color_eyre::install().unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <num>", &args[0]);
        std::process::exit(1);
    }
    match args[1].as_str() {
        "1" => println!("{:?}", aoc_1().unwrap()),
        "2" => println!("{:?}", aoc_2().unwrap()),
        "3" => println!("{:?}", aoc_3().unwrap()),
        "5" => println!("{:?}", aoc_5().unwrap()),
        "10" => println!("{:?}", aoc_10().unwrap()),
        "11" => println!("{:?}", aoc_11().unwrap()),
        "14" => println!("{:?}", aoc_14().unwrap()),
        "16" => println!("{:?}", aoc_16().unwrap()),
        "17" => println!("{:?}", aoc_17().unwrap()),
        _ => {
            println!("unrecognized num: {}", &args[1]);
            std::process::exit(1);
        }
    }
}
