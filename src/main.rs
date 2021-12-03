use std::env;
use std::fs;
use std::fmt;
use crate::challenge1::challenge1;

mod challenge1;
mod challenge2;
mod challenge3;
mod challenge4;
mod challenge5;
mod challenge6;

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
            println!("{:?}", challenge2(&contents));
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
        _ => {
            println!("Unknown challenge no.");
        }
    }
}
