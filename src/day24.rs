fn get_index(c: char) -> usize {
    match c {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => panic!("Unknown char: {}", c),
    }
}

fn run_program(program: &str, input: &[u8]) -> [i64; 4] {
    let mut input_index = 0;

    let mut state = [0; 4];

    let mut stream = program.split(&['\n', ' '][..]);
    while let Some(command) = stream.next() {
        if command.is_empty() { continue; }

        let a = stream.next().unwrap().chars().next().unwrap();
        if command == "inp" {
            state[get_index(a)] = input[input_index] as i64;
            input_index += 1;
            continue;
        }

        let a_val = state[get_index(a)];

        let b = stream.next().unwrap();
        // Either b is a variable, or it's a literal.
        let b_val = if b.len() == 1 && "wxyz".contains(b) {
            state[get_index(b.chars().next().unwrap())]
        } else {
            b.parse().unwrap()
        };

        state[get_index(a)] = match command {
            "add" => a_val + b_val,
            "mul" => a_val * b_val,
            "div" => a_val / b_val,
            "mod" => a_val % b_val,
            "eql" => (a_val == b_val) as i64,
            _ => { panic!("Unknown command: {}", command) }
        };
    }

    state
}

fn num_to_digits(num: u64) -> [u8; 14] {
    let mut w = num;
    let mut column = 1;
    let mut digits = [0u8; 14];

    for j in 1..15 {
        let digit = (w % (column * 10)) / column;
        digits[14 - j] = digit as u8;
        column *= 10;
        w -= digit;
    }

    digits
}

pub fn part1(program: &str) {
    let mut i = 99999u64;
    // let mut i = 99999999999999u64;

    while i > 0 {
        let model_num = num_to_digits(i);

        // if model_num.contains(&0) { continue; }

        let state = run_program(program, &model_num);

        if let 0 = state[get_index('z')] {
            println!("Found: {}", i);
        }

        i -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn negate_example() {
        let program = "inp x\nmul x -1";

        assert_eq!(-1, run_program(program, &[1])[get_index('x')]);
        assert_eq!(-24, run_program(program, &[24])[get_index('x')]);
        assert_eq!(0, run_program(program, &[0])[get_index('x')]);
    }

    #[test]
    fn three_times_bigger_example() {
        let program = "\
inp z
inp x
mul z 3
eql z x";

        assert_eq!(1, run_program(program, &[1, 3])[get_index('z')]);
        assert_eq!(1, run_program(program, &[2, 6])[get_index('z')]);
        assert_eq!(0, run_program(program, &[1, 2])[get_index('z')]);
    }

    #[test]
    fn to_binary() {
        let program = "\
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";

        assert_eq!([0, 1, 1, 0], run_program(program, &[6]));
        assert_eq!([0, 1, 0, 1], run_program(program, &[37]));
        assert_eq!([1, 1, 1, 1], run_program(program, &[15]));
        assert_eq!([0, 0, 0, 0], run_program(program, &[0]));
        assert_eq!([0, 0, 0, 0], run_program(program, &[32]));
    }

    #[test]
    fn test_num_to_digits() {
        assert_eq!(
            [1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4],
            num_to_digits(12345123451234)
        );
    }
}