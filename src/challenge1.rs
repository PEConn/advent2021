// TODO: Make generic
pub fn count_increases(values: &Vec<u32>) -> usize {
    // Pair each item with the next item, then add 1 for each
    // pair where the first is smaller than the second.
    values.iter().zip(values.iter().skip(1))
        .fold(0, |acc, (a, b)| acc + if a < b { 1 } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let input = vec![];
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
