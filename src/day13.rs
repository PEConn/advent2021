use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn flipped(&self, fold: &Fold) -> Coord {
        let mut x = self.x;
        let mut y = self.y;

        // Let's say we're folding (2, 2) along x = 4.
        // So, (x, y) = (2, 2) and fold_x = 4.
        // To find the new x, we need the distance between the current x and the fold: (x - fold_x).
        // Then we subtract that from the fold, giving: fold_x - (x - fold_x).
        // Which simplifies to: 2*fold_x - x.

        match fold {
            Fold::AlongX(fold_x) => {
                if x > *fold_x { x = (*fold_x) * 2 - x; }
            }

            Fold::AlongY(fold_y) => {
                if y > *fold_y { y = (*fold_y) * 2 - y; }
            }
        }

        Coord { x, y }
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