#[derive(Debug, PartialEq)]
pub struct Position(i32, i32);

#[derive(Debug, PartialEq)]
pub struct Displacement(i32, i32);

fn follow_command(command: &str) -> Result<Displacement, String> {
    let mut iter = command.split(' ');
    let direction = iter.next();
    let distance = iter.next();

    if direction.is_none() || distance.is_none() {
        return Err(format!("Poorly formed command: {}", command));
    }

    let direction = direction.unwrap();
    let distance : Result<i32, _> = distance.unwrap().parse();

    if distance.is_err() {
        return Err(format!("Poorly formed distance: {}", command));
    }

    let distance = distance.unwrap();

    match direction {
        "forward" => Ok(Displacement(distance, 0)),
        "up"      => Ok(Displacement(0, -distance)),
        "down"    => Ok(Displacement(0, distance)),
        _         => Err(format!("Unknown direction: {}", direction)),
    }
}

fn calculate_position(displacements: &[Displacement]) -> Position {
    displacements.iter().
        fold(Position(0, 0), |pos, dis| Position(pos.0 + dis.0, pos.1 + dis.1))
}

pub fn follow_commands(input: &str) -> Result<Position, String> {
    let displacements : Result<Vec<Displacement>, _> = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(follow_command)
        .collect();

    displacements.map(|x| calculate_position(&x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displace_up() {
        assert_eq!(Ok(Displacement(0, -3)), follow_command("up 3"));
    }

    #[test]
    fn displace_down() {
        assert_eq!(Ok(Displacement(0, 4)), follow_command("down 4"));
    }

    #[test]
    fn displace_forward() {
        assert_eq!(Ok(Displacement(5, 0)), follow_command("forward 5"));
    }

    #[test]
    fn displace_invalid() {
        assert!(follow_command("forward").is_err());
        assert!(follow_command("forward six").is_err());
        assert!(follow_command("foo 5").is_err());
    }
    
    #[test]
    fn empty() {
        let input = "";
        assert_eq!(Ok(Position(0, 0)), follow_commands(&input));
    }

    #[test]
    fn basic() {
        let input = "forward 2\ndown 2\nup 1";
        assert_eq!(Ok(Position(2, 1)), follow_commands(&input));
    }

    #[test]
    fn basic2() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert_eq!(Ok(Position(15, 10)), follow_commands(&input));
    }
}

