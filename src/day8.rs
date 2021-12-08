pub fn part1(input: &str) -> Result<i32, String> {
    let total_unique_digits : i32 = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let outputs = line
                .split('|')
                .skip(1)
                .next()
                .unwrap()
                .split(' ')
                .filter(|x| !x.is_empty());

            outputs.filter(|x| {
                let len = x.len();
                len == 2 || len == 4 || len == 3 || len == 7
            }).count() as i32
        }).sum();

    Ok(total_unique_digits)
}

fn remove_letter(input: &str, letter: char) -> String {
    input.chars().filter(|x| *x != letter).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_example_part1() {
        let input = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(Ok(26), part1(&input));
    }

    #[test]
    fn dev() {
        let input = ["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"];

        let one = input.iter().filter(|x| x.len() == 2).next().unwrap();
        let four = input.iter().filter(|x| x.len() == 4).next().unwrap();
        let seven = input.iter().filter(|x| x.len() == 3).next().unwrap();

        let two_or_three_or_five : Vec<&&str> = input.iter().filter(|x| x.len() == 5).collect();
        let zero_or_six_or_nine : Vec<&&str> = input.iter().filter(|x| x.len() == 6).collect();

        println!("{:?}", (one, seven));

        // a is the only segment in 7, but not in 1.
        let a = seven.chars().filter(|x| !one.contains(*x)).next().unwrap();
        println!("We know that {:?} is a.", a);

        // if you combine 4 and 7, then subtract that from 9, we get segment g.
        let mut g = 'x';
        for number in zero_or_six_or_nine {
            let s: String = number.chars().filter(|x| !four.contains(*x) && !seven.contains(*x)).collect();
            if s.len() == 1 {
                g = s.chars().next().unwrap();
            }
        }
        println!("We know that {:?} is g.", g);

        // let b = remove_letter(one, a).chars().next().unwrap();
        // println!("We know that {:?} is b.", b);

        // for x in input {
        //     let x = remove_letter(x, a);
        //     println!("{:?}", (x.len(), x));
        // }
    }
}