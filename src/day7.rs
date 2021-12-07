pub fn part1(input: &str) -> Result<i64, String> {
    Ok(calculate_fuel_cost(input, part1_cost))
}

pub fn part2(input: &str) -> Result<i64, String> {
    Ok(calculate_fuel_cost(input, part2_cost))
}

fn calculate_fuel_cost(input: &str, cost_function: fn(i64, i64) -> i64) -> i64 {
    // According to https://math.stackexchange.com/a/3092043 what I'm looking for here is just
    // a median of the list, but I didn't figure that out, so I'll try to come up with a
    // method myself. (This comment is for part #1 only.)

    // We know that there's going to be a low point which climbs higher to each side - so we don't
    // need to look at every value, we could just do a binary search. However, since the current
    // solution seems fast enough, let's not bother with that.
    let input : Vec<i64> = input
        .split(&[',', '\n'][..])
        .filter(|x| !x.is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    // We can just iterate from 0 until the fuel starts increasing.
    let mut position = 0;
    let mut cost = i64::MAX;

    loop {
        let current_cost = input.iter()
            .map(|x| cost_function(position, *x))
            .sum();

        if current_cost > cost {
            break;
        }

        cost = current_cost;
        position += 1;
    }

    cost
}

fn part1_cost(a: i64, b: i64) -> i64 {
    i64::abs(a - b)
}

fn part2_cost(a: i64, b: i64) -> i64 {
    // 1 -> 1             = 1
    // 2 -> 1 + 2         = 3
    // 3 -> 1 + 2 + 3     = 6
    // 4 -> 1 + 2 + 3 + 4 = 10
    // cost = dist * (dist + 1) / 2

    let dist = i64::abs(a - b);
    dist * (dist + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_example_part1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(Ok(37_i64), part1(&input));
    }

    #[test]
    fn given_example_part2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(Ok(168_i64), part2(&input));
    }

    #[test]
    fn part2_costs() {
        assert_eq!(1, part2_cost(0, 1));
        assert_eq!(3, part2_cost(0, 2));
        assert_eq!(6, part2_cost(0, 3));
        assert_eq!(10, part2_cost(0, 4));

        assert_eq!(10, part2_cost(2, 6));
        assert_eq!(10, part2_cost(6, 2));
    }
}