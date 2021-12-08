use std::env;
use std::fs;
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: <command> <challenge no>");
        return;
    }

    let challenge_no = &args[1];
    let filename = format!("input/input-{}.txt", challenge_no);

    let contents = fs::read_to_string(filename);

    if let Err(err) = contents {
        println!("{:?}", err);
        return;
    }

    let contents = contents.unwrap();

    match challenge_no.as_str() {
        "1" => {
            println!("{:?}", challenge1(&contents));
        }
        "2" => {
            println!("{:?}", challenge2::challenge2(&contents));
        }
        "3" => {
            println!("{:?}", challenge3::follow_commands(&contents));
        }
        "4" => {
            println!("{:?}", challenge4::follow_commands(&contents));
        }
        "5" => {
            println!("{:?}", challenge5::power_consumption(&contents));
        }
        "6" => {
            println!("{:?}", challenge6::challenge6(&contents));
        }
        "7" => {
            println!("{:?}", day4::part1(&contents));
        }
        "8" => {
            println!("{:?}", day4::part2(&contents));
        }
        "9" => {
            println!("{:?}", day5::part1(&contents));
        }
        "10" => {
            println!("{:?}", day5::part2(&contents));
        }
        "11" => {
            println!("{:?}", day6::part1(&contents));
        }
        "12" => {
            println!("{:?}", day6::part2(&contents));
        }
        "13" => {
            println!("{:?}", day7::part1(&contents));
        }
        "14" => {
            println!("{:?}", day7::part2(&contents));
        }
        "15" => {
            println!("{:?}", day8::part1(&contents));
        }
        _ => {
            println!("Unknown challenge no.");
        }
    }
}
