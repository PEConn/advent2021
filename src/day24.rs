use std::collections::{HashMap, HashSet};

type Int = i32;

fn get_index(c: char) -> usize {
    match c {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => panic!("Unknown char: {}", c),
    }
}

#[derive(Clone, Copy)]
enum Variable { W, X, Y, Z }

impl Variable {
    fn from(c: char) -> Variable {
        match c {
            'w' => Variable::W,
            'x' => Variable::X,
            'y' => Variable::Y,
            'z' => Variable::Z,
            _ => panic!("Unknown variable: {}", c),
        }
    }

    fn index(&self) -> usize {
        match self {
            Variable::W => 0,
            Variable::X => 1,
            Variable::Y => 2,
            Variable::Z => 3,
        }
    }

    fn value(&self, state: &[Int; 4]) -> Int {
        state[self.index()]
    }
}

#[derive(Clone, Copy)]
enum VarOrLiteral {
    Variable(Variable),
    Literal(i8)  // Looking at the input, this should cover us.
}

impl VarOrLiteral {
    fn value(&self, state: &[Int; 4]) -> Int {
        match self {
            VarOrLiteral::Variable(variable) => variable.value(state),
            VarOrLiteral::Literal(value) => *value as Int
        }
    }
}

#[derive(Clone, Copy)]
enum Command {
    Inp(Variable),
    Add(Variable, VarOrLiteral),
    Mul(Variable, VarOrLiteral),
    Div(Variable, VarOrLiteral),
    Mod(Variable, VarOrLiteral),
    Eql(Variable, VarOrLiteral),
}

fn parse_program(input: &str) -> Vec<Command> {
    let mut program = Vec::new();
    let mut stream = input.split(&['\n', ' '][..]);

    while let Some(command) = stream.next() {
        if command.is_empty() { continue; }

        let a_char = stream.next().unwrap().chars().next().unwrap();
        let a = Variable::from(a_char);

        if command == "inp" {
            program.push(Command::Inp(a));
            continue;
        }

        let b_str = stream.next().unwrap();
        // Either b is a variable, or it's a literal.
        let b: VarOrLiteral = if b_str.len() == 1 && "wxyz".contains(b_str) {
            VarOrLiteral::Variable(Variable::from(b_str.chars().next().unwrap()))
        } else {
            VarOrLiteral::Literal(b_str.parse().unwrap())
        };

        program.push(match command {
            "add" => Command::Add(a, b),
            "mul" => Command::Mul(a, b),
            "div" => Command::Div(a, b),
            "mod" => Command::Mod(a, b),
            "eql" => Command::Eql(a, b),
            _ => { panic!("Unknown command: {}", command) }
        });
    }

    program
}

fn run_program(program: &[Command], input: &[u8]) -> [Int; 4] {
    run_program_with_state(program, input, [0, 0, 0, 0])
}

fn run_program_with_state(program: &[Command], input: &[u8], state: [Int; 4]) -> [Int; 4] {
    let mut input_index = 0;
    let mut state = state;

    for command in program {
        match command {
            Command::Inp(v) => {
                state[v.index()] = input[input_index] as Int;
                input_index += 1;
            }
            Command::Add(v, o) => {
                state[v.index()] = v.value(&state) + o.value(&state);
            }
            Command::Mul(v, o) => {
                state[v.index()] = v.value(&state) * o.value(&state);
            }
            Command::Div(v, o) => {
                state[v.index()] = v.value(&state) / o.value(&state);
            }
            Command::Mod(v, o) => {
                state[v.index()] = v.value(&state) % o.value(&state);
            }
            Command::Eql(v, o) => {
                state[v.index()] = (v.value(&state) == o.value(&state)) as Int;
            }
        }
    }

    state
}

fn parse_and_run_program(program: &str, input: &[u8]) -> [Int; 4] {
    let program = parse_program(program);
    run_program(&program, input)
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
    // let mut i = 99999u64;
    // let mut i = 99999999999999u64;
    let program = parse_program(program);

    let mut program_parts : Vec<Vec<Command>> = Vec::new();

    for command in program.iter() {
        if let Command::Inp(_) = &command {
            program_parts.push(vec![*command]);
        } else {
            program_parts.last_mut().unwrap().push(*command);
        }
    }

    // This is the state that is carried over from one part to another.
    let mut possible_z_values: HashSet<Int> = HashSet::from([0]);
    // let mut best_for_z_at_position: HashMap<(usize, Int), u64> = HashMap::new();
    let mut best_for_prev_z: HashMap<Int, u64> = HashMap::new();

    for (n, part) in program_parts.iter().enumerate() {
        let mut next_z_values = HashSet::new();
        let mut best_for_z = HashMap::new();

        for i in 1..10 {
            for prev_z in possible_z_values.iter() {
                let z = run_program_with_state(part, &[i], [0, 0, 0, *prev_z])[3];

                // Comment out to get part 1
                // TODO: Split part 1 and 2.
                if best_for_z.contains_key(&z) { continue; }

                if n == 0 {
                    best_for_z.insert(z, i as u64);
                } else {
                    let prev = best_for_prev_z.get(prev_z).unwrap();
                    best_for_z.insert(z, prev * 10 + (i as u64));
                }

                next_z_values.insert(z);
            }
        }

        best_for_prev_z = best_for_z;
        possible_z_values = next_z_values;
        println!("Done {}, {}, {:?}", n, possible_z_values.len(), best_for_prev_z.get(&0));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn negate_example() {
        let program = "inp x\nmul x -1";

        assert_eq!(-1, parse_and_run_program(program, &[1])[get_index('x')]);
        assert_eq!(-24, parse_and_run_program(program, &[24])[get_index('x')]);
        assert_eq!(0, parse_and_run_program(program, &[0])[get_index('x')]);
    }

    #[test]
    fn three_times_bigger_example() {
        let program = "\
inp z
inp x
mul z 3
eql z x";

        assert_eq!(1, parse_and_run_program(program, &[1, 3])[get_index('z')]);
        assert_eq!(1, parse_and_run_program(program, &[2, 6])[get_index('z')]);
        assert_eq!(0, parse_and_run_program(program, &[1, 2])[get_index('z')]);
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

        assert_eq!([0, 1, 1, 0], parse_and_run_program(program, &[6]));
        assert_eq!([0, 1, 0, 1], parse_and_run_program(program, &[37]));
        assert_eq!([1, 1, 1, 1], parse_and_run_program(program, &[15]));
        assert_eq!([0, 0, 0, 0], parse_and_run_program(program, &[0]));
        assert_eq!([0, 0, 0, 0], parse_and_run_program(program, &[32]));
    }

    #[test]
    fn test_num_to_digits() {
        assert_eq!(
            [1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4],
            num_to_digits(12345123451234)
        );
    }

    #[test]
    fn dev() {
        let program = "\
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0";

        let program = parse_program(program);
        for z in 0..100 {
            for w in 1..10 {
                let z : i32 = z - 50;  // Make sure we work with negative numbers as well.
                let x = if (w as i32) == z % 26 + 14 { 0 } else { 1 };
                assert_eq!(x, run_program_with_state(&program, &[w], [0, 0, 0, z.into()])[1]);
            }
        }

        let program = "\
mul y 0
add y 25
mul y x
add y 1";
        let program = parse_program(program);
        for x in 0..2 {
            assert_eq!(25 * x + 1, run_program_with_state(&program, &[], [0, x, 0, 0])[2]);
        }

        let program = "\
mul y 0
add y w
add y 12
mul y x";
        let program = parse_program(program);
        for w in 1..10 {
            for x in 0..50 {
                let x = x - 50;  // Make sure we work with negative numbers as well.
                assert_eq!((w + 12) * x, run_program_with_state(&program, &[], [w, x, 0, 0])[2]);
            }
        }
    }

    #[test]
    fn test_sanity() {
        let mut best_for_z_at_position: HashMap<(usize, Int), u64> = HashMap::new();
        let mut possible_z : HashSet<Int> = HashSet::from([0]);

        for n in 0..6 {
            let mut pow_ten = 1;
            let mut next_possible_z = HashSet::new();

            for i in 1..10 {
                for z in possible_z.iter() {
                    let next_z = ((n as Int) * i * z) as Int;

                    if n == 0 {
                        best_for_z_at_position.insert((n, next_z), i as u64);
                    } else {
                        let prev = best_for_z_at_position.get(&(n - 1, *z)).unwrap();

                        let to_insert = (i as u64) * pow_ten + prev;
                        println!("Adding {} to {}: {}", i, prev, to_insert);
                        best_for_z_at_position.insert((n, next_z), to_insert);
                    }

                    next_possible_z.insert(next_z);
                }

            }

            pow_ten *= 10;

            possible_z = next_possible_z;
        }

        for ((n, z), input) in best_for_z_at_position {
            if n == 3 {
                println!("{}", input);
            }
        }
    }
}