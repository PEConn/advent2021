use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum SyntaxError {
    IllegalCharacter(char),
    MissingCharacters(String),
    None
}

impl SyntaxError {
    fn score(&self) -> i64{
        match self {
            SyntaxError::IllegalCharacter(')') => 3,
            SyntaxError::IllegalCharacter(']') => 57,
            SyntaxError::IllegalCharacter('}') => 1197,
            SyntaxError::IllegalCharacter('>') => 25137,
            SyntaxError::IllegalCharacter(c) => panic!("Unknown IllegalCharacter: {:?}", c),
            SyntaxError::MissingCharacters(string) => {
                let mut score = 0;

                for c in string.chars() {
                    score = score * 5 + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("Unknown MissingCharacter: {:?}", c)
                    }
                }

                score
            },
            SyntaxError::None => 0
        }
    }
}

fn check_line(input: &str) -> SyntaxError {
    let pairs = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    let mut stack : Vec<char> = Vec::new();

    for c in input.chars() {
        if pairs.contains_key(&c) {
            stack.push(c);
        } else {
            let expected = *pairs.get(&stack.pop().unwrap()).unwrap();

            if c != expected {
                return SyntaxError::IllegalCharacter(c)
            }
        }
    }

    if stack.is_empty() {
        return SyntaxError::None
    }

    let mut closing = String::new();
    while let Some(c) = stack.pop() {
        if let Some(expected) = pairs.get(&c) {
            closing.push(*expected);
        } else {
            panic!("Unknown character on stack {:?}", c);
        }
    }

    SyntaxError::MissingCharacters(closing)
}

pub fn part1(input: &str) -> i64 {
    let mut total = 0_i64;

    for line in input.lines() {
        let error = check_line(line);

        total += match error {
            SyntaxError::IllegalCharacter(_) => error.score(),
            _ => 0,
        }
    }

    total
}

pub fn part2(input: &str) -> i64 {
    let mut scores : Vec<i64> = Vec::new();

    for line in input.lines() {
        let error = check_line(line);

        match error {
            SyntaxError::MissingCharacters(_) => {
                scores.push(error.score());
            }
            _ => ()
        }
    }

    scores.sort();

    *scores.get(scores.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day10::SyntaxError::{IllegalCharacter, MissingCharacters, None};
    use super::*;

    #[test]
    fn no_syntax_errors() {
        assert_eq!(None, check_line("(())"));
        assert_eq!(None, check_line("([])"));
        assert_eq!(None, check_line("([][])"));
    }

    #[test]
    fn illegal_character() {
        assert_eq!(IllegalCharacter('}'), check_line("{([(<{}[<>[]}>{[]{[(<()>"));
        assert_eq!(IllegalCharacter(')'), check_line("[[<[([]))<([[{}[[()]]]"));
        assert_eq!(IllegalCharacter(']'), check_line("[{[{({}]{}}([{[{{{}}([]"));
        assert_eq!(IllegalCharacter(')'), check_line("[<(<(<(<{}))><([]([]()"));
        assert_eq!(IllegalCharacter('>'), check_line("<{([([[(<>()){}]>(<<{{"));
    }

    #[test]
    fn missing_characters() {
        assert_eq!(MissingCharacters("}}]])})]".into()), check_line("[({(<(())[]>[[{[]{<()<>>"));
        assert_eq!(MissingCharacters(")}>]})".into()), check_line("[(()[<>])]({[<{<<[]>>("));
        assert_eq!(MissingCharacters("}}>}>))))".into()), check_line("(((({<>}<{<{<>}{[]{[]{}"));
        assert_eq!(MissingCharacters("]]}}]}]}>".into()), check_line("{<[[]]>}<{[{[{[]{()[[[]"));
        assert_eq!(MissingCharacters("])}>".into()), check_line("<{([{{}}[<[[[<>{}]]]>[]]"));
    }

    #[test]
    fn score_missing_characters() {
        assert_eq!(288957, MissingCharacters("}}]])})]".into()).score());
        assert_eq!(5566, MissingCharacters(")}>]})".into()).score());
        assert_eq!(1480781, MissingCharacters("}}>}>))))".into()).score());
        assert_eq!(995444, MissingCharacters("]]}}]}]}>".into()).score());
        assert_eq!(294, MissingCharacters("])}>".into()).score());
    }

    const GIVEN_INPUT : &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn given_example_part1() {
        assert_eq!(26397, part1(GIVEN_INPUT));
    }

    #[test]
    fn given_example_part2() {
        assert_eq!(288957, part2(GIVEN_INPUT));
    }
}