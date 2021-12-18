use crate::day18::Token::{CloseBracket, OpenBracket, Number};
use std::fmt;
use std::fmt::{Formatter, Write};

type Num = u32;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Number(Num),
}

impl Token {
    fn add(&self, a: Num) -> Token {
        // TODO: Figure out a nicer way to type this.
        match self {
            Number(b) => Number(a + b),
            _ => panic!("Cannot add to a non-number: {:?}", self),
        }
    }

    fn get_num(&self) -> Num {
        // TODO: Figure out if I can keep the convenience but add some safety.
        match self {
            Number(n) => *n,
            _ => panic!("Called get_num on a non-number: {:?}", self),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OpenBracket => f.write_char('['),
            CloseBracket => f.write_char(']'),
            Number(n) => f.write_fmt(format_args!("{}", n)),
        }
    }
}

type SnailNumber = Vec<Token>;

fn parse(input: &str) -> SnailNumber {
    input.chars().filter_map(|c| {
        match c {
            '[' => Some(OpenBracket),
            ']' => Some(CloseBracket),
            ',' => None,
            other => other.to_digit(10).map(|d| Number(d))
        }
    }).collect()
}

fn add_to(a: &mut SnailNumber, b: SnailNumber) {
    // TODO: This seems ugly, is there a better way?
    let mut b = b;

    a.insert(0, Token::OpenBracket);
    a.append(&mut b);
    a.push(Token::CloseBracket);
}

fn add_to_previous_number(n: &mut SnailNumber, index: usize, num: Num) {
    n.iter_mut()
        .take(index - 1)
        .filter(|token| matches!(token, Number(_)))
        .last()
        .map(|token| {
           *token = token.add(num);
        });
}

fn add_to_next_number(n: &mut SnailNumber, index: usize, num: Num) {
    n.iter_mut()
        .skip(index + 1)
        .filter(|token| matches!(token, Number(_)))
        .next()
        .map(|token| {
            *token = token.add(num);
        });
}

fn explode(n: &mut SnailNumber) -> bool {
    let mut depth = 0;
    let mut i = 0;

    while i < n.len() {
        let current = n.get(i).unwrap();

        if let OpenBracket = current {
            if depth == 4 {
                let left = n.get(i+1).unwrap().get_num();
                let right = n.get(i+2).unwrap().get_num();

                add_to_previous_number(n, i + 1, left);
                add_to_next_number(n, i + 2, right);

                n.drain(i..(i+4));
                n.insert(i, Number(0));

                return true;
            }
        }

        match current {
            OpenBracket => { depth += 1; }
            CloseBracket => { depth -= 1; }
            Number(_) => {}
        }

        i += 1
    }

    return false;
}

fn split(n: &mut SnailNumber) -> bool {
    let mut i = 0;

    while i < n.len() {
        let current = n.get(i).unwrap().clone();

        match current {
            OpenBracket => {}
            CloseBracket => {}
            Number(value) => {
                if value >= 10 {
                    n.remove(i);
                    n.insert(i, OpenBracket);
                    n.insert(i + 1, Number(value / 2));
                    n.insert(i + 2, Number(value / 2 + value % 2));
                    n.insert(i + 3, CloseBracket);

                    return true;
                }
            }
        }

        i += 1;
    }

    return false;
}

fn reduce(n: &mut SnailNumber) {
    let mut changed = true;
    while changed {
        changed = explode(n);

        if !changed {
            changed = split(n);
        }
    }
}

fn add_and_reduce(a: &mut SnailNumber, b: SnailNumber) {
    add_to(a, b);
    reduce(a);
}

fn sum(numbers: &[SnailNumber]) -> SnailNumber {
    let mut total : SnailNumber = numbers.first().unwrap().clone();
    for number in numbers.iter().skip(1) {
        add_and_reduce(&mut total, number.clone());
    }

    total
}

pub fn part1(input: &str) -> Num {
    let v : Vec<SnailNumber> = input.lines().map(|line| parse(line)).collect();
    magnitude(&mut sum(&v).iter())
}

pub fn part2(input: &str) -> Num {
    let v : Vec<SnailNumber> = input.lines().map(|line| parse(line)).collect();

    let mut best_magnitude = 0 as Num;
    for i in 0..v.len() {
        for j in 0..v.len() {
            if i == j { continue; }

            let m = magnitude(&mut sum(
                &[
                    v.get(i).unwrap().clone(),
                    v.get(j).unwrap().clone()
                ][..]).iter());

            best_magnitude = Num::max(m, best_magnitude)
        }
    }
    best_magnitude
}

fn magnitude<'a, I: Iterator<Item = &'a Token>>(stream: &mut I) -> Num {
    let token = stream.next().unwrap();
    match token {
        OpenBracket => {
            let left = magnitude(stream);
            let right = magnitude(stream);

            stream.next();  // Get rid of the Closing Bracket.

            3 * left + 2 * right
        }
        CloseBracket => {
            panic!("Shouldn't come across closing bracket.")
        }
        Number(_) => { token.get_num() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_explode() {
        let mut num = parse("[[[[[9,8],1],2],3],4]");
        explode(&mut num);
        assert_eq!(parse("[[[[0,9],2],3],4]"), num);

        let mut num = parse("[7,[6,[5,[4,[3,2]]]]]");
        explode(&mut num);
        assert_eq!(parse("[7,[6,[5,[7,0]]]]"), num);

        let mut num = parse("[[6,[5,[4,[3,2]]]],1]");
        explode(&mut num);
        assert_eq!(parse("[[6,[5,[7,0]]],3]"), num);

        let mut num = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        explode(&mut num);
        assert_eq!(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), num);

        let mut num = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        explode(&mut num);
        assert_eq!(parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"), num);
    }

    #[test]
    fn test_add() {
        let mut num = parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        add_to(&mut num, parse("[1,1]"));
        reduce(&mut num);
        assert_eq!(parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), num);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(29, magnitude(&mut parse("[9,1]").iter()));
        assert_eq!(21, magnitude(&mut parse("[1,9]").iter()));
        assert_eq!(129, magnitude(&mut parse("[[9,1],[1,9]]").iter()));
        assert_eq!(143, magnitude(&mut parse("[[1,2],[[3,4],5]]").iter()));
        assert_eq!(1384, magnitude(&mut parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").iter()));
        assert_eq!(445, magnitude(&mut parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").iter()));
        assert_eq!(791, magnitude(&mut parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").iter()));
        assert_eq!(1137, magnitude(&mut parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").iter()));
        assert_eq!(3488, magnitude(&mut parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").iter()));
    }

    #[test]
    fn example1() {
        let total = sum(&vec![
            parse("[1,1]"),
            parse("[2,2]"),
            parse("[3,3]"),
            parse("[4,4]"),
        ]);
        let expected = parse("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(expected, total);
    }

    #[test]
    fn example2() {
        let total = sum(&vec![
            parse("[1,1]"),
            parse("[2,2]"),
            parse("[3,3]"),
            parse("[4,4]"),
            parse("[5,5]"),
            parse("[6,6]"),
        ]);
        let expected = parse("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(expected, total);
    }

    #[test]
    fn given_example_part1() {
        let input = "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

        let expected =
            parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");

        let v : Vec<SnailNumber> = input.lines().map(|line| parse(line)).collect();
        let result = sum(&v);
        assert_eq!(expected, result);

        assert_eq!(3488, part1(input));
    }
}