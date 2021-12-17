use sscanf::scanf;

// TODO: Settle on Coord vs Position
#[derive(Debug)]
struct Position { x: i32, y: i32 }

#[derive(Debug)]
struct Velocity { x: i32, y: i32 }

#[derive(Debug)]
struct Rect { top: i32, left: i32, bottom: i32, right: i32 }

impl Rect {
    fn contains(&self, point: &Position) -> bool {
        point.x >= self.left && point.x <= self.right
            && point.y >= self.bottom && point.y <= self.top
    }
}

fn parse(input: &str) -> Rect {
    let parsed =
        scanf!(input, "target area: x={}..{}, y={}..{}\n", i32, i32, i32, i32);

    let (x1, x2, y1, y2) = parsed.unwrap();

    Rect {
        top: i32::max(y1, y2),
        left: i32::min(x1, x2),
        bottom: i32::min(y1, y2),
        right: i32::max(x1, x2),
    }
}

fn step(position: &Position, velocity: &Velocity) -> (Position, Velocity) {
    let new_velocity = Velocity {
        x: velocity.x - velocity.x.signum(),
        y: velocity.y - 1,
    };

    let new_position = Position {
        x: position.x + velocity.x,
        y: position.y + velocity.y,
    };

    (new_position, new_velocity)
}

fn brute_force(target: &Rect) -> Vec<Velocity> {
    // max x velocity = target's RHS, since we'll overshoot immediately.
    // max y velocity = target's bottom, since when the probe gets to y=0 on the way down, it will
    // be travelling at negative the initial y velocity, and again we'll overshoot immediately.
    let min_x = 1;
    let max_x = target.right;
    let min_y = target.bottom;
    let max_y = -target.bottom;

    println!("{:?}", (max_x, max_y));

    let mut solutions = vec![];

    for x in min_x..(max_x + 1) {
        for y in min_y..(max_y + 1) {
            let mut velocity = Velocity { x, y };
            let mut position = Position { x: 0, y: 0 };

            loop {
                if target.contains(&position) {
                    solutions.push(Velocity { x, y });
                    break;
                }

                if position.x > target.right || position.y < target.bottom {
                    break;
                }

                let result = step(&position, &velocity);
                position = result.0;
                velocity = result.1;
            }
        }
    }

    solutions
}

fn height_of_peak(velocity: &Velocity) -> i32 {
    let y = velocity.y;
    (y + 1) * y / 2
}

pub fn part1(input: &str) -> i32 {
    let solutions = brute_force(&parse(input));
    let best_velocity = solutions.iter().max_by_key(|v| v.y).unwrap();

    height_of_peak(best_velocity)
}

pub fn part2(input: &str) -> usize {
    brute_force(&parse(input)).len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_example_part1() {
        let input = "target area: x=20..30, y=-10..-5\n";

        assert_eq!(45, part1(input));
    }

    #[test]
    fn given_example_part2() {
        let input = "target area: x=20..30, y=-10..-5\n";

        assert_eq!(112, part2(input));
    }
}