mod util;

mod aoc_1b;
mod aoc_2;
mod aoc_4;
mod aoc_5;
mod aoc_6;
mod aoc_9;

fn main() {
    color_eyre::install().unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <num>", &args[0]);
        std::process::exit(1);
    }
    match args[1].as_str() {
        "1b" => println!("{:?}", aoc_1b::aoc_1b().unwrap()),
        "2" => println!("{:?}", aoc_2::aoc_2().unwrap()),
        "4" => println!("{:?}", aoc_4::aoc_4().unwrap()),
        "5" => println!("{:?}", aoc_5::aoc_5().unwrap()),
        "6" => println!("{:?}", aoc_6::aoc_6().unwrap()),
        "9" => println!("{:?}", aoc_9::aoc_9().unwrap()),
        _ => {
            println!("unrecognized num: {}", &args[1]);
            std::process::exit(1);
        }
    }
}
