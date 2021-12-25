use std::fmt::{Debug, Formatter, Write};

struct Map {
    contents: Vec<Vec<char>>,
    width: usize,
    height: usize
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    East,
    South,
}

impl Map {
    fn parse(input: &str) -> Map {
        let contents : Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();
        Map {
            height: contents.len(),
            width: contents.first().unwrap().len(),
            contents
        }
    }

    fn step(&self) -> Option<Map> {
        if let Some(moved_east) = self.step_dir(&Direction::East) {
            // TODO: There's probably a nicer way to do this.
            if let Some(moved_south) = moved_east.step_dir(&Direction::South) {
                println!("Moved east and south");
                Some(moved_south)
            } else {
                println!("Moved east");
                Some(moved_east)
            }
        } else {
            println!("Moved south");
            self.step_dir(&Direction::South)
        }
    }

    fn step_dir(&self, dir: &Direction) -> Option<Map> {
        let mut new_contents = vec![vec!['.'; self.width]; self.height];
        let mut changed = false;

        for y in 0..self.height {
            for x in 0..self.width {
                let (next_x, next_y) = match dir {
                    Direction::East => ((x + 1) % self.width, y),
                    Direction::South => (x, (y + 1) % self.height),
                };

                let c = match dir {
                    Direction::East => '>',
                    Direction::South => 'v',
                };

                if self.contents[y][x] == c && self.contents[next_y][next_x] == '.' {
                    changed = true;
                    new_contents[next_y][next_x] = self.contents[y][x];
                } else if self.contents[y][x] != '.' {
                    new_contents[y][x] = self.contents[y][x];
                }
            }
        }

        if changed {
            Some(Map {
                contents: new_contents,
                width: self.width,
                height: self.height,
            })
        } else {
            None
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.contents.iter() {
            for c in line.iter() {
                f.write_char(*c)?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

pub fn part1(input: &str) -> u32 {
    let mut map = Map::parse(input);
    let mut step_count = 1;

    while let Some(next_map) = map.step() {
        map = next_map;
        step_count += 1;
    }

    step_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn moves_one_by_one() {
        let step0 = Map::parse("...>>>>>...");
        let step1 = Map::parse("...>>>>.>..");
        let step2 = Map::parse("...>>>.>.>.");

        assert_eq!(step1.contents, step0.step_dir(&Direction::East).unwrap().contents);
        assert_eq!(step2.contents, step1.step_dir(&Direction::East).unwrap().contents);
    }

    #[test]
    fn moves_over_edge() {
        let step0 = Map::parse("...>");
        let step1 = Map::parse(">...");

        assert_eq!(step1.contents, step0.step_dir(&Direction::East).unwrap().contents);
    }

    #[test]
    fn cant_move() {
        let map = Map::parse("\
....
>>>>
....");
        assert!(map.step_dir(&Direction::East).is_none());
        assert!(map.step_dir(&Direction::South).is_none());
    }

    #[test]
    fn basic() {
        let step0 = Map::parse("\
....
.>v.
.v>.
....
");
        let step1 = Map::parse("\
....
.>..
..v>
.v..");
        let step2 = Map::parse("\
.v..
..>.
>...
..v.");

        assert_eq!(step1.contents, step0.step().unwrap().contents);
        assert_eq!(step2.contents, step1.step().unwrap().contents);
    }

    #[test]
    fn given_example() {
        let input = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        let step0 = Map::parse(input);
        let step1 = Map::parse("\
....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v");

        assert_eq!(step1.contents, step0.step().unwrap().contents);
        assert_eq!(58, part1(input));
    }
}