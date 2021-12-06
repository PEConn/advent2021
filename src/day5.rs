use std::collections::HashMap;

pub fn part1(input: &str) -> Result<i32, String> {
    count_overlaps(input, false)
}

pub fn part2(input: &str) -> Result<i32, String> {
    count_overlaps(input, true)
}

fn count_overlaps(input: &str, diagonals: bool) -> Result<i32, String> {
    let mut vents : HashMap<(i32, i32), i32> = HashMap::new();

    input.lines().for_each(|line| {
        let mut parts = line.split(&[',', '-', '>', ' '][..])
            .filter(|x| !x.is_empty());

        let x1 : i32 = parts.next().unwrap().parse().unwrap();
        let y1 : i32 = parts.next().unwrap().parse().unwrap();
        let x2 : i32 = parts.next().unwrap().parse().unwrap();
        let y2 : i32 = parts.next().unwrap().parse().unwrap();

        let x_distance = i32::abs(x2 - x1);
        let y_distance = i32::abs(y2 - y1);

        let x_velocity = (x2 - x1).signum();
        let y_velocity = (y2 - y1).signum();

        let steps = i32::max(x_distance, y_distance);

        if x_velocity != 0 && y_velocity != 0 && !diagonals { return; }

        for i in 0..(steps + 1) {
            let pos = (x1 + i * x_velocity, y1 + i * y_velocity);
            vents.insert(pos, *vents.get(&pos).unwrap_or(&0) + 1);
        }
    });

    Ok(vents.iter().filter(|(_, count)| **count >= 2).count() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
       assert_eq!(Ok(5), part1(&EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(Ok(12), part2(&EXAMPLE));
    }
}