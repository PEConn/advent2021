use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

// TODO: Figure out what requires Ord/Eq and what requires PartialOrd/PartialEq
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord {x, y}
    }
}

struct Map {
    width: usize,
    height: usize,
    contents: Vec<Vec<i32>>,
}

impl Map {
    fn new(contents: Vec<Vec<i32>>) -> Map {
        Map {
            width: contents.first().unwrap().len(),
            height: contents.len(),
            contents
        }
    }

    // TODO: Should this be "from" instead? Find some guidelines for when to implement "from".
    fn parse(input: &str) -> Map {
        Map::new(input
            .lines()
            .map(|line| {
                line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
            }).collect())
    }

    fn get(&self, c: &Coord) -> i32 {
        *self.contents.get(c.y).unwrap().get(c.x).unwrap()
    }

    /// Returns a list of coordinates of all neighbours.
    fn neighbours(&self, c: &Coord) -> Vec<Coord> {
        let mut neighbours : Vec<Coord> = Vec::new();

        if c.x > 0 { neighbours.push(Coord::new(c.x - 1, c.y)) }
        if c.y > 0 { neighbours.push(Coord::new(c.x, c.y - 1)) }
        if c.x < (self.width - 1) { neighbours.push(Coord::new(c.x + 1, c.y))}
        if c.y < (self.height - 1) { neighbours.push(Coord::new(c.x, c.y + 1))}

        neighbours
    }
}

fn find_least_risk_path(map: &Map) -> i32 {
    let mut visited : HashSet<Coord> = HashSet::new();
    let mut to_visit : BinaryHeap<(Reverse<i32>, Coord)> = BinaryHeap::new();
    let start = Coord::new(0, 0);
    let end = Coord::new(map.width - 1, map.height - 1);

    visited.insert(start);
    to_visit.push((Reverse(0), start));

    while let Some((Reverse(risk), next_coord)) = to_visit.pop() {
        if next_coord == end {
            return risk;
        }

        for neighbour in map.neighbours(&next_coord) {
            if visited.contains(&neighbour) { continue; }

            to_visit.push((Reverse(risk + map.get(&neighbour)), neighbour));
            visited.insert(neighbour);
        }
    }

    panic!("Didn't reach the end!");
}

fn extend_map(map: &Map) -> Map {
    // Extend vertically.
    let mut new_contents : Vec<Vec<i32>> = Vec::new();

    // Extend the rows across.
    for row in map.contents.iter() {
        // Insert the existing part of the map.
        new_contents.push(row.clone());

        let current_row = new_contents.last_mut().unwrap();
        for i in 1..5 {
            // When a number goes over 9 it wraps around to 1. If it wrapped around to 0 then we
            // could just do (*x + 1) % 10. Since it wraps around to 1 instead, we need to do a
            // modulo 9, but offset by 1, hence we minus 1 before the modulo and then add it back
            // afterwards.

            for x in row.iter() {
                current_row.push((*x + i - 1) % 9 + 1);
            }
        }
    }

    let extended_right = new_contents.clone();

    // Extend the rows downwards.
    for i in 1..5 {
        for row in extended_right.iter() {
            new_contents.push(row.iter()
                .map(|x| (*x + i - 1) % 9 + 1)
                .collect())
        }
    }

    Map::new(new_contents)
}

pub fn part1(input: &str) -> i32 {
    find_least_risk_path(&Map::parse(input))
}

pub fn part2(input: &str) -> i32 {
    find_least_risk_path(&extend_map(&Map::parse(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT : &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn given_example_part1() {
        let map = Map::parse(INPUT);
        assert_eq!(40, find_least_risk_path(&map));
    }

    #[test]
    fn given_example_part2() {
        let map = Map::parse(INPUT);
        let map = extend_map(&map);
        assert_eq!(315, find_least_risk_path(&map));
    }
}