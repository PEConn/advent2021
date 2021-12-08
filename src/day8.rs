use std::collections::HashSet;

pub fn part1(input: &str) -> Result<i32, String> {
    let total_unique_digits : i32 = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let outputs = line
                .split('|')
                .skip(1)
                .next().unwrap()
                .split(' ')
                .filter(|x| !x.is_empty());

            outputs.filter(|x| {
                let len = x.len();
                len == 2 || len == 4 || len == 3 || len == 7
            }).count() as i32
        }).sum();

    Ok(total_unique_digits)
}

pub fn part2(input: &str) -> Result<i32, String> {
    let mut total = 0;

    input.lines().filter(|x| !x.is_empty()).for_each(|line| {
        let mut split = line.split("|");
        let inputs : Vec<&str> = split.next().unwrap().split(' ').filter(|x| !x.is_empty()).collect();
        let outputs : Vec<&str> = split.next().unwrap().split(' ').filter(|x| !x.is_empty()).collect();

        total += solve(&inputs, &outputs);
    });

    Ok(total)
}

fn minus(a: &HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
    a.difference(b).cloned().collect()
}

fn plus(a: &HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
    a.union(b).cloned().collect()
}

fn sum(sets: &[&HashSet<char>]) -> HashSet<char> {
    let mut result : HashSet<char> = HashSet::new();
    for set in sets {
        result.insert(*set.iter().next().unwrap());
    }
    result
    // TODO: Make this approach work.
    // sets.iter().flatten().cloned().collect()
}

fn solve(input: &[&str], output: &[&str]) -> i32 {
    let input: Vec<HashSet<char>> = input.iter().map(|word| word.chars().collect()).collect();

    let one = input.iter().filter(|x| x.len() == 2).next().unwrap();
    let four = input.iter().filter(|x| x.len() == 4).next().unwrap();
    let seven = input.iter().filter(|x| x.len() == 3).next().unwrap();
    let eight : &HashSet<char> = &vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().collect();

    // a is the only segment in 7 but not in 1.
    let a = minus(seven, &one);

    let two_three_five : Vec<&HashSet<char>> = input.iter().filter(|x| x.len() == 5).collect();
    let zero_six_nine : Vec<&HashSet<char>> = input.iter().filter(|x| x.len() == 6).collect();

    // If we combine 4 and 7, then subtract that from 9, we get a single segment, g.
    // If we combine 4 and 7, then subtract that from 0 or 6, we get two segments.
    let nine : &HashSet<char> = zero_six_nine.iter().filter(|num| {
        num.difference(&plus(four, seven)).count() == 1
    }).next().unwrap();
    let g = minus(nine, &plus(four, seven));

    // 9 contains all the segments of 3, but not all the elements of 2 or 5.
    // 9 contains all the segments of 3 and 5, but not of 2.
    let two : &HashSet<char> = two_three_five.iter().filter(|x| !nine.is_superset(x)).next().unwrap();

    // 9 contains all segments except for e.
    // 8 contains all segments.
    // So the only segment in 8 but not in 9 is e.
    let e = minus(eight, nine);

    // f is the only segment that 1 contains that 2 doesn't.
    let f = minus(one, two);

    // 1 is made up of segments c and f.
    let c = minus(one, &f);

    // 2 contains segments a, c, d, e, g.
    let d = minus(two, &sum(&vec![&a, &c, &e, &g]));

    // b is the last segment we don't know.
    let b = minus(eight, &sum(&vec![&a, &c, &d, &e, &f, &g]));

    // Complete the rest of the numbers:
    let zero = minus(eight, &d);
    let six = minus(eight, &c);
    let five = minus(eight, &plus(&c, &e));
    let three = minus(eight, &plus(&b, &e));

    let outputs : Vec<HashSet<char>> = output.iter().map(|word| word.chars().collect()).collect();

    // TODO: Standardize these as either references or not.
    // TODO: Make this nicer.
    let mut result = 0;
    for o in outputs {
        let num = if o == zero { 0 }
        else if o == *one { 1 }
        else if o == *two { 2 }
        else if o == three { 3 }
        else if o == *four { 4 }
        else if o == five { 5 }
        else if o == six { 6 }
        else if o == *seven { 7 }
        else if o == *eight { 8 }
        else if o == *nine { 9 }
        else { panic!("Couldn't figure out a number") };

        result = result * 10 + num;
    }

    result
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
    fn given_example_part2_1() {
        let input = ["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"];
        let output = ["cdfeb", "fcadb", "cdfeb", "cdbaf"];

        assert_eq!(5353, solve(&input, &output));
    }

    #[test]
    fn given_example_part2_2() {
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

        assert_eq!(Ok(61229), part2(&input));
    }
}
