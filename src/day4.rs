use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    contents: Vec<i32>,
}

impl From<&str> for Board {
    fn from(s: &str) -> Self {
        Board {
            contents: s
                .split(char::is_whitespace)
                .filter(|s| !s.is_empty())
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        }
    }
}

impl Board {
    fn get(&self, x: usize, y: usize) -> &i32 {
        self.contents.get(x * 5 + y).unwrap()
    }

    fn get_row(&self, x: usize) -> Vec<&i32> {
        (0..5).map(|y| self.get(x, y)).collect()
    }

    fn get_column(&self, y:usize) -> Vec<&i32> {
        (0..5).map(|x| self.get(x, y)).collect()
    }

    fn has_won(&self, drawn_numbers: &HashSet<i32>) -> bool {
        for x in 0..5 {
            if self.get_row(x).iter().all(|x| drawn_numbers.contains(x)) {
                return true;
            }
        }

        for y in 0..5 {
            if self.get_column(y).iter().all(|x| drawn_numbers.contains(x)) {
                return true;
            }
        }

        return false;
    }

    fn sum_unmarked_numbers(&self, drawn_numbers: &HashSet<i32>) -> i32 {
        self.contents.iter().filter(|x| !drawn_numbers.contains(x)).sum()
    }
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<Board>) {
    let draws = input.lines().next().unwrap();
    let draws : Vec<i32> = draws.split(',')
        .map(str::parse)
        .map(Result::unwrap).collect();

    let board_lines = input.lines()
        .skip(1)
        .filter(|s| !s.is_empty());

    let mut boards: Vec<Board> = Vec::new();
    let mut board_str = String::new();
    let mut counter = 0;

    for line in board_lines {
        board_str.push_str(line);
        board_str.push(' ');
        counter += 1;

        if counter == 5 {
            boards.push(Board::from(board_str.as_str()));
            board_str = String::new();
            counter = 0;
        }
    }

    (draws, boards)
}

pub fn part1(input: &str) -> Result<i32, String> {
    let (draws, boards) = parse_input(input);

    let mut drawn_numbers: HashSet<i32> = HashSet::new();
    for draw in draws.iter() {
        drawn_numbers.insert(*draw);

        for board in boards.iter() {
            if board.has_won(&drawn_numbers) {
                return Ok(board.sum_unmarked_numbers(&drawn_numbers) * draw);
            }
        }
    }

    Err(String::from("Could not find a winner"))
}

pub fn part2(input: &str) -> Result<i32, String> {
    let (draws, mut boards) = parse_input(input);

    let mut drawn_numbers: HashSet<i32> = HashSet::new();
    for draw in draws.iter() {
        drawn_numbers.insert(*draw);

        if boards.len() == 1 && boards.get(0).unwrap().has_won(&drawn_numbers) {
            return Ok(draw * boards.get(0).unwrap().sum_unmarked_numbers(&drawn_numbers));
        }

        boards.retain(|board| !board.has_won(&drawn_numbers));

    }

    Err(String::from("Could not find a winner"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
";

    const GIVEN_INPUT: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    fn check_vec_equals(exp: &Vec<i32>, act: &Vec<&i32>) {
        for i in 0..exp.len() {
            assert_eq!(exp.get(i).unwrap(), *act.get(i).unwrap());
        }
    }

    #[test]
    fn get_rows() {
        let board = Board::from(INPUT);

        check_vec_equals(&vec![22, 13, 17, 11,  0], &board.get_row(0));
        check_vec_equals(&vec![ 8,  2, 23,  4, 24], &board.get_row(1));
        check_vec_equals(&vec![21,  9, 14, 16,  7], &board.get_row(2));
        check_vec_equals(&vec![ 6, 10,  3, 18,  5], &board.get_row(3));
        check_vec_equals(&vec![ 1, 12, 20, 15, 19], &board.get_row(4));
    }

    #[test]
    fn get_columns() {
        let board = Board::from(INPUT);

        check_vec_equals(&vec![22,  8, 21,  6,  1], &board.get_column(0));
        check_vec_equals(&vec![13,  2,  9, 10, 12], &board.get_column(1));
        check_vec_equals(&vec![17, 23, 14,  3, 20], &board.get_column(2));
        check_vec_equals(&vec![11,  4, 16, 18, 15], &board.get_column(3));
        check_vec_equals(&vec![ 0, 24,  7,  5, 19], &board.get_column(4));
    }

    #[test]
    fn has_won() {
        let board = Board::from(INPUT);

        let mut drawn_numbers = HashSet::new();

        drawn_numbers.insert(8);  assert!(!board.has_won(&drawn_numbers));
        drawn_numbers.insert(2);  assert!(!board.has_won(&drawn_numbers));
        drawn_numbers.insert(23); assert!(!board.has_won(&drawn_numbers));
        drawn_numbers.insert(4);  assert!(!board.has_won(&drawn_numbers));
        drawn_numbers.insert(24); assert!(board.has_won(&drawn_numbers));
    }

    #[test]
    fn sum_unmarked_numbers() {
        let board = Board::from(INPUT);

        let mut drawn_numbers = HashSet::new();

        drawn_numbers.insert(22);
        drawn_numbers.insert(13);
        drawn_numbers.insert(17);
        drawn_numbers.insert(11);
        drawn_numbers.insert(0);

        let exp = 8 +  2 + 23 +  4 + 24
                    + 21 +  9 + 14 + 16 +  7
                    +  6 + 10 +  3 + 18 +  5
                    +  1 + 12 + 20 + 15 + 19;
        assert_eq!(exp, board.sum_unmarked_numbers(&drawn_numbers));
    }

    #[test]
    fn given_example_part1() {
        assert_eq!(Ok(4512), part1(&GIVEN_INPUT));
    }

    #[test]
    fn given_example_part2() {
        assert_eq!(Ok(1924), part2(&GIVEN_INPUT));
    }
}