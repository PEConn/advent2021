
pub fn part1(input: &str) -> Result<i32, String> {
    Ok(simulate_lanternfish(input, 80) as i32)
}

pub fn part2(input: &str) -> Result<i64, String> {
    Ok(simulate_lanternfish(input, 256))
}

fn simulate_lanternfish(input: &str, days: i32) -> i64 {
    let mut fish = [0_i64; 9];
    input.split(&[',', '\n'][..])
        .filter(|x| !x.is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .for_each(|x: usize| fish[x] += 1);

    for _ in 0..days {
        let num_new_parents = fish[0];

        fish[0] = fish[1];
        fish[1] = fish[2];
        fish[2] = fish[3];
        fish[3] = fish[4];
        fish[4] = fish[5];
        fish[5] = fish[6];
        fish[6] = fish[7];
        fish[7] = fish[8];

        fish[8] = num_new_parents;
        fish[6] += num_new_parents;
    }

    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_example() {
        let input = "3,4,3,1,2";

        assert_eq!(5, simulate_lanternfish(&input, 0));
        assert_eq!(5, simulate_lanternfish(&input, 1));
        assert_eq!(6, simulate_lanternfish(&input, 2));
        assert_eq!(7, simulate_lanternfish(&input, 3));
        assert_eq!(9, simulate_lanternfish(&input, 4));
        assert_eq!(10, simulate_lanternfish(&input, 5));
        assert_eq!(10, simulate_lanternfish(&input, 6));
        assert_eq!(10, simulate_lanternfish(&input, 7));
        assert_eq!(10, simulate_lanternfish(&input, 8));
        assert_eq!(11, simulate_lanternfish(&input, 9));

        assert_eq!(Ok(5934), part1(&input));
        assert_eq!(Ok(26984457539), part2(&input));
    }
}