use std::env;
use std::fs;
use std::path::Path;

use crate::challenge1::challenge1;

mod challenge1;
mod challenge2;
mod challenge3;
mod challenge4;
mod challenge5;
mod challenge6;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod vector;
mod day20;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: <command> <challenge no>");
        return;
    }

    let challenge_no: Result<u32, _> = args[1].parse();

    if challenge_no.is_err() {
        println!("Could not parse challenge number");
        return;
    }

    let challenge_no = challenge_no.unwrap();
    let mut filename = format!("input/input-{}.txt", challenge_no);

    // The part 2 challenge may use the input from the preceding part 1 challenge. If we're on a
    // part 2 (the challenge number is even) and there's no input, use the input from the part 1.
    if challenge_no % 2 == 0 && !Path::new(&filename).exists() {
        // Try the file for the previous challenge.
        filename = format!("input/input-{}.txt", challenge_no - 1);
    }

    let contents = fs::read_to_string(filename);

    if let Err(err) = contents {
        println!("{:?}", err);
        return;
    }

    let contents = contents.unwrap();

    // TODO: Standardize some API for all the challenges.
    match challenge_no {
        1 => { println!("{:?}", challenge1(&contents)) }
        2 => { println!("{:?}", challenge2::challenge2(&contents)) }
        3 => { println!("{:?}", challenge3::follow_commands(&contents)) }
        4 => { println!("{:?}", challenge4::follow_commands(&contents)) }
        5 => { println!("{:?}", challenge5::power_consumption(&contents)) }
        6 => { println!("{:?}", challenge6::challenge6(&contents)) }
        7 => { println!("{:?}", day4::part1(&contents)) }
        8 => { println!("{:?}", day4::part2(&contents)) }
        9 => { println!("{:?}", day5::part1(&contents)) }
        10 => { println!("{:?}", day5::part2(&contents)) }
        11 => { println!("{:?}", day6::part1(&contents)) }
        12 => { println!("{:?}", day6::part2(&contents)) }
        13 => { println!("{:?}", day7::part1(&contents)) }
        14 => { println!("{:?}", day7::part2(&contents)) }
        15 => { println!("{:?}", day8::part1(&contents)) }
        16 => { println!("{:?}", day8::part2(&contents)) }
        17 => { println!("{:?}", day9::part1(&contents)) }
        18 => { println!("{:?}", day9::part2(&contents)) }
        19 => { println!("{:?}", day10::part1(&contents)) }
        20 => { println!("{:?}", day10::part2(&contents)) }
        21 => { println!("{:?}", day11::part1(&contents)) }
        22 => { println!("{:?}", day11::part2(&contents)) }
        23 => { println!("{:?}", day12::part1(&contents)) }
        24 => { println!("{:?}", day12::part2(&contents)) }
        25 => { println!("{:?}", day13::part1(&contents)) }
        26 => { println!("{}", day13::part2(&contents)) }
        27 => { println!("{}", day14::part1(&contents)) }
        28 => { println!("{}", day14::part2(&contents)) }
        29 => { println!("{}", day15::part1(&contents)) }
        30 => { println!("{}", day15::part2(&contents)) }
        31 => { println!("{}", day16::part1(&contents)) }
        32 => { println!("{}", day16::part2(&contents)) }
        33 => { println!("{}", day17::part1(&contents)) }
        34 => { println!("{}", day17::part2(&contents)) }
        35 => { println!("{}", day18::part1(&contents)) }
        36 => { println!("{}", day18::part2(&contents)) }
        37 => { println!("{}", day19::part1(&contents)) }
        38 => { println!("{}", day19::part2(&contents)) }
        39 => { println!("{}", day20::part1(&contents)) }
        40 => { println!("{}", day20::part2(&contents)) }
        _ => { println!("Unknown challenge no."); }
    }
}
