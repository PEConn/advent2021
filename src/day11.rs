use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

impl PartialEq<Coord> for (usize, usize) {
    fn eq(&self, other: &Coord) -> bool {
        *self == (other.x, other.y)
    }
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

    fn inc(&mut self, c: &Coord) {
        *self.contents.get_mut(c.y).unwrap().get_mut(c.x).unwrap() += 1
    }

    fn reset(&mut self, c: &Coord) {
        *self.contents.get_mut(c.y).unwrap().get_mut(c.x).unwrap() = 0
    }

    fn neighbours(&self, c: &Coord) -> Vec<Coord> {
        //// Returns a list of coordinates of all neighbours.

        let mut neighbours : Vec<Coord> = Vec::new();

        for i in -1..2_i32 {
            for j in -1..2_i32 {
                if i == 0 && j == 0 { continue; }

                let x = c.x as i32 + i;
                let y = c.y as i32 + j;

                if x < 0 { continue; }
                if y < 0 { continue; }
                if x >= self.width as i32 { continue; }
                if y >= self.height as i32 { continue; }

                neighbours.push(Coord::new(x as usize, y as usize));
            }
        }

        neighbours
    }

    fn step(&mut self) -> usize {
        // Increment all cells.
        for x in 0..self.width {
            for y in 0..self.height {
                self.inc(&Coord::new(x, y))
            }
        }

        // Trigger flashes.
        let mut flashed : HashSet<Coord> = HashSet::new();
        loop {
            let mut changed = false;

            for x in 0..self.width {
                for y in 0..self.height {
                    let c = Coord::new(x, y);

                    if !flashed.contains(&c) && self.get(&c) > 9 {
                        for neighbour in self.neighbours(&c) {
                            self.inc(&neighbour);
                        }

                        flashed.insert(c);
                        changed = true;
                    }
                }
            }

            if !changed { break; }
        }

        // Reset the cells that have flashed.
        for coord in flashed.iter() {
            self.reset(coord)
        }

        flashed.len()
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(&Coord::new(x, y)));
            }
            println!();
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut map = Map::parse(input);
    let mut total_flashes = 0;

    for _ in 0..100 {
        total_flashes += map.step() as i32;
    }

    total_flashes
}

pub fn part2(input: &str) -> i32 {
    let mut map = Map::parse(input);
    let mut step = 1;

    loop {
        if map.step() == map.width * map.height {
            break;
        }

        step += 1
    }

    step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours_corner1() {
        let map = Map::parse("123\n456\n789");

        let mut neighbours = map.neighbours(&Coord::new(0, 0));
        neighbours.sort();
        assert_eq!(vec![(0, 1), (1, 0), (1, 1)], neighbours);
    }

    #[test]
    fn test_neighbours_corner2() {
        let map = Map::parse("123\n456\n789");

        let mut neighbours = map.neighbours(&Coord::new(2, 2));
        neighbours.sort();
        assert_eq!(vec![(1, 1), (1, 2), (2, 1)], neighbours);
    }

    #[test]
    fn basic_example() {
        let input = "\
11111
19991
19191
19991
11111";
        let mut map = Map::parse(input);

        assert_eq!(9, map.step());
        assert_eq!(Map::parse("\
34543
40004
50005
40004
34543").contents, map.contents);

        assert_eq!(0, map.step());
        assert_eq!(Map::parse("\
45654
51115
61116
51115
45654").contents, map.contents);
    }

    #[test]
    fn given_example_part1() {
        let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        assert_eq!(1656, part1(input));
    }

    #[test]
    fn given_example_part2() {
        let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        assert_eq!(195, part2(input));
    }
}