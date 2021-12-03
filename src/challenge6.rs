fn get_most_common_bit(input: &Vec<&str>, index: usize) -> char {
    //// Gives the most common bit at position `index` across all entries in
    //// the vector. Returns '1' on a tie.
    let count = input.len();

    let num_ones = input.iter()
        .map(|x| x.chars().nth(index).unwrap())
        .filter(|&x| x == '1')
        .count();

    // Compare num_ones*2 with count instead of comparing num_ones with
    // count/2 so we don't need to worry about rounding.
    if num_ones * 2 >= count { '1' } else { '0' }
}

fn flip(input: char) -> char {
    if input == '1' { '0' } else { '1' }
}

#[derive(Clone, Debug)]
enum Rating {
    Oxygen,
    CO2,
}

fn get_rating(input: &str, rating: &Rating) -> u32 {
    let mut input : Vec<&str> = input.lines().collect();
    let mut index = 0;

    loop {
        let desired_bit = match rating {
            Rating::Oxygen => get_most_common_bit(&input, index),
            Rating::CO2 => flip(get_most_common_bit(&input, index)),
        };

        input.retain(|x| x.chars().nth(index).unwrap() == desired_bit);
        
        if input.len() == 1 { break; }
        index += 1
    }

    u32::from_str_radix(input.get(0).unwrap(), 2).unwrap()
}

pub fn challenge6(input: &str) -> u32 {
    let oxygen = get_rating(&input, &Rating::Oxygen);
    let co2 = get_rating(&input, &Rating::CO2);
    oxygen * co2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

        assert_eq!(23, get_rating(&input, &Rating::Oxygen));
        assert_eq!(10, get_rating(&input, &Rating::CO2));
        assert_eq!(230, challenge6(&input));
    }
}
