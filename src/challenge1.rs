pub fn count_increases(values: &[impl PartialOrd]) -> u32 {
    // Pair each item with the next item, then count the pairs where the first element is smaller
    // than the second.
    values.iter().zip(values.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count() as u32
}

pub fn challenge1(input: &str) -> Result<u32, String> {
    let lines: Result<Vec<u32>, _> = input.lines().map(str::parse).collect();

    lines.map(|x| count_increases(&x))
        .map_err(|_| String::from("Could not parse number"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let input : Vec<i32> = vec![];
        assert_eq!(0, count_increases(&input));
    }

    #[test]
    fn basic() {
        let input = vec![2, 3, 4, 5, 6, 3, 2];
        assert_eq!(4, count_increases(&input));
    }

    #[test]
    fn equal() {
        let input = vec![4, 4, 4, 4, 4, 4, 4];
        assert_eq!(0, count_increases(&input));
    }
}
