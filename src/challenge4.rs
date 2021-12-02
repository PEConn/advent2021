#[derive(Debug, PartialEq)]
pub struct Position(i32, i32);

type Aim = i32;

#[derive(Debug, PartialEq)]
pub struct State {
    pos: Position,
    aim: Aim,
}

impl State {
    pub fn new() -> State {
        State {
            pos: Position(0, 0),
            aim: 0,
        }
    }
}

fn follow_command(command: &str, state: State) -> Result<State, String> {
    let mut iter = command.split(' ');

    let direction = iter.next().ok_or("Empty command")?;
    let value = iter.next().ok_or("Value missing")?;
    let value : i32 = value.parse().map_err(|_| "Cannot parse value")?;

    match direction {
        "forward" => {
            let x = state.pos.0 + value;
            let y = state.pos.1 + value * state.aim;
            Ok(State { pos: Position(x, y), ..state })
        },
        "up"      => Ok(State { aim: state.aim - value, ..state }),
        "down"    => Ok(State { aim: state.aim + value, ..state }),
        _         => Err(format!("Unknown direction: {}", direction)),
    }
}

pub fn follow_commands(input: &str) -> Result<State, String> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .try_fold(State::new(), |state, line| follow_command(line, state))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_state(x: i32, y: i32, aim: i32, state: &State) {
        assert_eq!(State {
            pos: Position(x, y),
            aim: aim,
        }, *state);
    }

    #[test]
    fn bad_command() {
        assert!(follow_command("abc", State::new()).is_err());
    }

    #[test]
    fn bad_value() {
        assert!(follow_command("abc de", State::new()).is_err());
    }

    #[test]
    fn bad_direction() {
        assert!(follow_command("abc 3", State::new()).is_err());
    }

    #[test]
    fn displace_up() {
        assert_state(0, 0, -4, &follow_command("up 4", State::new()).unwrap());
    }

    #[test]
    fn displace_down() {
        assert_state(0, 0, 4, &follow_command("down 4", State::new()).unwrap());
    }

    #[test]
    fn displace_forward() {
        assert_state(3, 0, 0, &follow_command("forward 3", State::new()).unwrap());
    }

    #[test]
    fn displace_up_down() {
        let commands = "down 4\nup 2";
        assert_state(0, 0, 2, &follow_commands(&commands).unwrap());
    }

    #[test]
    fn displace_forward_with_aim() {
        let commands = "down 4\nup 2\nforward 2";
        assert_state(2, 4, 2, &follow_commands(&commands).unwrap());
    }

    #[test]
    fn given_example() {
        let commands = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert_state(15, 60, 10, &follow_commands(&commands).unwrap());
    }
}

