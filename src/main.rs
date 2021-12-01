use std::env;
use std::fs;

mod challenge1;
use challenge1::count_increases;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: <command> <challenge no> <input file name>");
        return;
    }

    let challenge_no = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename);

    if let Err(err) = contents {
        println!("{:?}", err);
        return;
    }

    let contents = contents.unwrap();

    match challenge_no.as_str() {
        "1" => {
            let contents: Result<Vec<u32>, _> =
                contents.lines().map(|x| x.parse()).collect();
            let result = contents.map(|x| count_increases(&x));
            println!("{:?}", result);
        }
        _ => {
            println!("Unknown challenge no.");
        }
    }
}
