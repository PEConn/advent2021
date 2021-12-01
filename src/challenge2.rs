pub fn count_triplet_increases(values: &Vec<u32>) -> usize {
    let iter_a = values.windows(3);
    let iter_b = values.windows(3).skip(1);
    // This could be more efficient - we don't need to do a sum on
    // every iteration, we could have a running total where we add
    // the new element and subtract the old one.
    iter_a.zip(iter_b).fold(0, |acc, (a, b)| {
        let sum_a : u32 = a.iter().sum();
        let sum_b : u32 = b.iter().sum();
        acc + if sum_a < sum_b { 1 } else { 0 }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let input = vec![];
        assert_eq!(0, count_triplet_increases(&input));
    }

    #[test]
    fn basic() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, count_triplet_increases(&input));
    }

    #[test]
    fn equal() {
        let input = vec![4, 4, 4, 4, 4, 4, 4];
        assert_eq!(0, count_triplet_increases(&input));
    }
}
