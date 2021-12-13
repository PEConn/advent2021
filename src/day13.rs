use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn flipped(&self, fold: &Fold) -> Coord {
        match fold {
            Fold::AlongX(fold_x) => {
                let mut x = self.x;

                if x > *fold_x { x = (*fold_x) * 2 - x; }

                Coord { x, ..*self }
            }

            Fold::AlongY(fold_y) => {
                let mut y = self.y;

                if y > *fold_y { y = (*fold_y) * 2 - y; }

                Coord { y, ..*self }
            }
        }
    }
}

#[derive(Debug)]
enum Fold {
    AlongX(usize),
    AlongY(usize),
}

fn parse(input: &str) -> (HashSet<Coord>, Vec<Fold>) {
    let mut points : HashSet<Coord> = HashSet::new();
    let mut folds : Vec<Fold> = Vec::new();

    for line in input.lines() {
        if line.is_empty() { continue; }

        if line.starts_with("fold along y=") {
            let y = line.split('=').skip(1).next().unwrap().parse().unwrap();
            folds.push(Fold::AlongY(y))
        } else if line.starts_with("fold along x=") {
            let x = line.split('=').skip(1).next().unwrap().parse().unwrap();
            folds.push(Fold::AlongX(x))
        } else {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            points.insert(Coord { x, y });
        }
    }

    (points, folds)
}

pub fn part1(input: &str) -> usize {
    let (mut points, folds) = parse(input);

    let first_fold = folds.first().unwrap();
    points = points.iter().map(|point| point.flipped(&first_fold)).collect();
    points.len()
}

pub fn part2(input: &str) -> String {
    let (mut points, folds) = parse(input);

    for fold in folds {
        points = points.iter().map(|point| point.flipped(&fold)).collect();
    }

    let mut output = String::new();

    let max_x = points.iter().map(|point| point.x).max().unwrap();
    let max_y = points.iter().map(|point| point.y).max().unwrap();

    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            if points.contains(&Coord{ x, y}) {
                output.push('#');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dev() {
        let input = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        assert_eq!(17, part1(input));
    }
}