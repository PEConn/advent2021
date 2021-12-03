pub fn most_common_bits(input: &str) -> String {
    // A Vec of zeroes as long as the first line of the file.
    let zeroes : Vec<u32> = input.lines().next().unwrap()
        .chars().map(|_| 0).collect();

    let num_lines = input.lines().count() as u32;
    let bit_counts = input.lines()
        .map(|x| x.chars())
        .fold(zeroes, |acc, x|
              acc.iter().zip(x).map(inc_if_1).collect());

    bit_counts.iter()
        .map(|x| {
            // Compare x*2 with num_lines instead of comparing x with
            // num_lines/2 so we don't have to deal with rounding.
            if x*2 >= num_lines { '1' } else { '0' }
        }).collect()
}

pub fn power_consumption(input: &str) -> u32 {
    let gamma = most_common_bits(input);
    let epsilon : String = gamma.chars()
        .map(|x| if x == '0' { '1' } else { '0' })
        .collect();

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

fn inc_if_1(state: (&u32, char)) -> u32 {
    let (count, elem) = state;
    count + if elem == '1' { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = "000\n001\n001";
        // gamma = 0b110 = 6
        // epsilon = 0b001 = 1

        assert_eq!(6, power_consumption(&input));
    }

    #[test]
    fn given_example() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

        assert_eq!(198, power_consumption(&input));
    }
}
