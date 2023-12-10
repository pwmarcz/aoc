mod aoc_1b;
mod aoc_4;
use aoc_1b::aoc_1b;
use aoc_4::aoc_4;

fn main() {
    color_eyre::install().unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <num>", &args[0]);
        std::process::exit(1);
    }
    match args[1].as_str() {
        "1b" => println!("{:?}", aoc_1b().unwrap()),
        "4" => println!("{:?}", aoc_4().unwrap()),
        _ => {
            println!("unrecognized num: {}", &args[1]);
            std::process::exit(1);
        }
    }
}
