pub fn count_increases(values: &[impl PartialOrd]) -> u32 {
    // Pair each item with the next item, then count the pairs where the first element is smaller
    // than the second.
    values.iter().zip(values.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count() as u32
}

pub fn part1(input: &str) -> Result<u32, String> {
    let lines: Result<Vec<u32>, _> = input.lines().map(str::parse).collect();

    lines.map(|x| count_increases(&x))
        .map_err(|_| String::from("Could not parse number"))
}

pub fn count_triplet_increases(values: &[u32]) -> u32 {
    let iter_a = values.windows(3);
    let iter_b = values.windows(3).skip(1);
    // This could be more efficient - we don't need to do a sum on
    // every iteration, we could have a running total where we add
    // the new element and subtract the old one.
    iter_a.zip(iter_b)
        .filter(|(a, b)| {
            let sum_a : u32 = a.iter().sum();
            let sum_b : u32 = b.iter().sum();
            sum_a < sum_b
        })
        .count() as u32
}

pub fn part2(input: &str) -> Result<u32, String> {
    let lines: Result<Vec<u32>, _> = input.lines().map(str::parse).collect();

    lines.map(|x| count_triplet_increases(&x))
        .map_err(|_| String::from("Could not parse number"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_part1() {
        let input : Vec<i32> = vec![];
        assert_eq!(0, count_increases(&input));
    }

    #[test]
    fn basic_part1() {
        let input = vec![2, 3, 4, 5, 6, 3, 2];
        assert_eq!(4, count_increases(&input));
    }

    #[test]
    fn equal_part1() {
        let input = vec![4, 4, 4, 4, 4, 4, 4];
        assert_eq!(0, count_increases(&input));
    }

    #[test]
    fn empty_part2() {
        let input = vec![];
        assert_eq!(0, count_triplet_increases(&input));
    }

    #[test]
    fn basic_part2() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, count_triplet_increases(&input));
    }

    #[test]
    fn equal_part2() {
        let input = vec![4, 4, 4, 4, 4, 4, 4];
        assert_eq!(0, count_triplet_increases(&input));
    }
}
