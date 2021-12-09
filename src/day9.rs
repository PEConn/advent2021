use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
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

    fn neighbours(&self, c: &Coord) -> Vec<Coord> {
        //// Returns a list of coordinates of all neighbours.

        let mut neighbours : Vec<Coord> = Vec::new();

        if c.x > 0 { neighbours.push(Coord::new(c.x - 1, c.y)) }
        if c.y > 0 { neighbours.push(Coord::new(c.x, c.y - 1)) }
        if c.x < (self.width - 1) { neighbours.push(Coord::new(c.x + 1, c.y))}
        if c.y < (self.height - 1) { neighbours.push(Coord::new(c.x, c.y + 1))}

        neighbours
    }

    fn lower_neighbours(&self, c: &Coord) -> Vec<Coord> {
        //// Returns a list of coordinates for all the neighbours that have a lower value.

        let value = self.get(c);
        let mut neighbours = self.neighbours(c);
        neighbours.retain(|n| self.get(n) < value);

        neighbours
    }
}

pub fn part1(input: &str) -> i32 {
    calculate_risk(&Map::parse(&input))
}

fn calculate_risk(map: &Map) -> i32 {
    let mut risk = 0;

    for x in 0..map.width {
        for y in 0..map.height {
            let c = Coord::new(x, y);
            let current = map.get(&c);

            if map.lower_neighbours(&c).len() == 0 {
                risk += 1 + current
            }
        }
    }

    risk
}

fn find_low_point_and_update_path(map: &Map, coord: &Coord,
                                  destinations: &mut HashMap<Coord, Coord>) {
    //// Finds the low point that a given coordinate will flow to. Updates `destinations` so that
    //// each coordinate on the path to the basin now points to the coordinates of the basin.

    // Find the low point
    let mut current = *coord;
    let mut path : Vec<Coord> = Vec::new();

    loop {
        if destinations.contains_key(&current) {
            // We've reached a point we already know the low point for.
            current = *destinations.get(&current).unwrap();
            break;
        }

        let lower_neighbours = map.lower_neighbours(&current);
        if lower_neighbours.len() == 0 {
            // We've reached the low point
            break;
        } else {
            path.push(current);
            current = *lower_neighbours.first().unwrap();
        }
    }

    let low_point = current;

    // Update the values for the path.
    for c in path {
        destinations.insert(c, low_point);
    }
}

fn calculate_basin_sizes(map: &Map) -> Vec<i64> {
    let mut destinations: HashMap<Coord, Coord> = HashMap::new();

    for x in 0..map.width {
        for y in 0..map.height {
            let c = Coord{x, y};

            // Locations of height 9 do not count as being in any basin,
            // all other locations will always be part of exactly one basin.
            if map.get(&c) == 9 { continue }

            find_low_point_and_update_path(&map, &c, &mut destinations);
        }
    }

    let mut basin_sizes: HashMap<Coord, i64> = HashMap::new();
    for (_, sink) in destinations.iter() {
        basin_sizes.insert(*sink, basin_sizes.get(sink).as_deref().unwrap_or(&1) + 1);
    }

    basin_sizes.values().cloned().collect()
}

pub fn part2(input: &str) -> i64 {
    let map = Map::parse(&input);
    let mut sizes = calculate_basin_sizes(&map);
    sizes.sort_by(|a, b| b.cmp(a));

    sizes.iter().take(3).cloned().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_example_part1() {
        let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(15, calculate_risk(&Map::parse(&input)));
    }

    #[test]
    fn given_example_part2() {
        let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(1134, part2(&input));
    }

    #[test]
    fn test_optimization() {
        let input = "\
0123
1234";

        part2(&input);
    }
}