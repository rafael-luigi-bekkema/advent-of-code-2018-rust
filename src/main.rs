#![feature(drain_filter)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let case = if args.len() >= 2 { &args[1] } else { "1" };

    match case {
        "1" => {
            println!("day01a: {}", day01::a());
            println!("day01b: {}", day01::b());
        }
        "2" => {
            println!("day02a: {}", day02::a());
            println!("day02b: {}", day02::b());
        }
        "3" => {
            println!("day03a: {}", day03::a());
            println!("day03b: {}", day03::b());
        }
        "4" => {
            println!("day04a: {}", day04::a());
            println!("day04b: {}", day04::b());
        }
        "5" => {
            println!("day05a: {}", day05::a());
            println!("day05a: {}", day05::b());
        }
        "6" => {
            println!("day06a: {}", day06::a());
            println!("day06a: {}", day06::b());
        }
        _ => {
            println!("unknown case");
        }
    }
}
