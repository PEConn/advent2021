use std::collections::HashMap;
use std::cmp;

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

        let x_min = cmp::min(x1, x2);
        let x_max = cmp::max(x1, x2);
        let y_min = cmp::min(y1, y2);
        let y_max = cmp::max(y1, y2);

        if x1 == x2 {
            for y in y_min..(y_max + 1) {
                vents.insert((x1, y), *vents.get(&(x1, y)).unwrap_or(&0) + 1);
            }
        } else if y1 == y2 {
            for x in x_min..(x_max + 1) {
                vents.insert((x, y1), *vents.get(&(x, y1)).unwrap_or(&0) + 1);
            }
        } else if diagonals {
            // Four cases
            // Diagonals like \
            // (0, 0) -> (2, 2)
            // (2, 2) -> (0, 0)
            // Diagonals like /
            // (2, 0) -> (0, 2)
            // (0, 2) -> (2, 0)
            let steps = x_max - x_min;

            for i in 0..(steps + 1) {
                let point = if (x1 < x2) == (y1 < y2) {
                    // Diagonal like \
                    (x_min + i, y_min + i)
                } else {
                    // Diagonal like /
                    (x_min + i, y_max - i)
                };
                vents.insert(point, *vents.get(&point).unwrap_or(&0) + 1);
            }
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