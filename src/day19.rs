// Note: To profile, cargo flamegraph --root --output fg.svg -- 37

use std::collections::HashSet;

use sscanf::scanf;
use crate::vector::{Vector, VectorTransform, ROTATIONS, manhatten_distance};

type Beacon = Vector;

struct Scanner {
    beacons: HashSet<Beacon>,
}

fn parse_beacon(line: &str) -> Beacon {
    let parsed = scanf!(line, "{},{},{}", i32, i32, i32).unwrap();
    Beacon::new(parsed.0, parsed.1, parsed.2)
}

impl Scanner {
    fn new(beacons: HashSet<Beacon>) -> Scanner {
        Scanner { beacons }
    }

    fn parse(input: &str) -> Scanner {
        Scanner::new(input.lines().skip(1).map(parse_beacon).collect())
    }

    fn beacons_with_offset(&self, offset: &Vector) -> Vec<Beacon> {
        self.beacons.iter().map(|b| b.add(offset)).collect()
    }
}

fn parse(input: &str) -> Vec<Scanner> {
    input.split("\n\n").map(|chunk| Scanner::parse(chunk)).collect()
}

/// Gives a list of possible positions of s2 relative to s1.
fn possible_offsets(s1: &Scanner, s2: &Scanner) -> HashSet<Beacon> {
    let mut offsets = HashSet::new();

    for b1 in s1.beacons.iter() {
        for b2 in s2.beacons.iter() {
            offsets.insert(b1.minus(b2));
        }
    }

    offsets
}

#[derive(Debug)]
struct Operations {
    eq: u64,
    rotations: u64,
}

impl Operations {
    fn new() -> Operations { Operations { eq: 0, rotations: 0 } }
}

fn find_match(s1: &Scanner, s2: &Scanner, threshold: u32, ops: &mut Operations)
        -> Option<(Vector, &'static VectorTransform)> {
    for rotation in ROTATIONS.iter() {
        // TODO: Cut down on rotations, eg, rotate just the base vector or cache them.
        let s2 = Scanner {
            beacons: s2.beacons.iter().map(|b| {
                ops.rotations += 1;
                rotation.apply(b)
            }).collect()
        };

        for offset in possible_offsets(&s1, &s2) {
            let mut num_matches = 0;

            // Before change: 5.4s.
            // After change: 0.8.
            for b2 in s2.beacons_with_offset(&offset).iter() {
                ops.eq += 1;

                if s1.beacons.contains(b2) {
                    num_matches += 1;

                    if num_matches >= threshold {
                        return Some((offset, rotation));
                    }
                }
            }
        }
    }

    None
}

fn combine_scanners(scanners: &mut Vec<Scanner>) -> (Scanner, Vec<Vector>) {
    let mut base = scanners.pop().unwrap();
    let mut ops = Operations::new();

    let mut scanner_positions: Vec<Vector> = Vec::new();
    scanner_positions.push(Vector::new(0, 0, 0));

    while !scanners.is_empty() {
        let mut indexes_to_remove : Vec<usize> = Vec::new();
        println!("Considering {} scanners.", scanners.len());

        for i in 0..scanners.len() {
            let other = scanners.get(i).unwrap();
            if let Some((offset, rotation)) = find_match(&base, other, 12, &mut ops) {
                println!("Found match with {}", i);

                scanner_positions.push(offset);
                indexes_to_remove.push(i);

                // add other to base.
                for beacon in other.beacons.iter() {
                    base.beacons.insert(rotation.apply(beacon).add(&offset));
                }
            }
        }

        println!("Pass finished.");
        for i  in indexes_to_remove.iter().rev() {
            scanners.remove(*i);
        }
    }

    println!("{:?}", ops);

    (base, scanner_positions)
}

pub fn part1(input: &str) -> usize {
    let mut scanners = parse(input);
    let (base, _) = combine_scanners(&mut scanners);
    base.beacons.len()
}

pub fn part2(input: &str) -> i32 {
    let mut scanners = parse(input);
    let (_, scanner_positions) = combine_scanners(&mut scanners);

    // TODO: Is there some method to get the cartesian product of two iterators?
    let mut max_dist = 0;
    for a in scanner_positions.iter() {
        for b in scanner_positions.iter() {
            max_dist = i32::max(max_dist, manhatten_distance(a, b));
        }
    }
    max_dist
}

// TODO: Profile
// TODO: Parallelize
// TODO: See if there are some logic optimizations I can make.

#[cfg(test)]
mod test {
    use crate::day19inputs::{INPUT, INPUT_LESS};
    use super::*;

    #[test]
    fn test_parse() {
        let scanners = parse(INPUT);

        assert_eq!(5, scanners.len());
        assert_eq!(25, scanners.get(0).unwrap().beacons.len());
        assert_eq!(25, scanners.get(1).unwrap().beacons.len());
        assert_eq!(26, scanners.get(2).unwrap().beacons.len());
        assert_eq!(25, scanners.get(3).unwrap().beacons.len());
        assert_eq!(26, scanners.get(4).unwrap().beacons.len());
    }

    #[test]
    fn smaller_example() {
        let mut scanners = parse(INPUT_LESS);
        let (base, _) = combine_scanners(&mut scanners);
        assert_eq!(52, base.beacons.len());
    }

    #[test]
    fn given_example_part1() {
        let mut scanners = parse(INPUT);

        let (base, _) = combine_scanners(&mut scanners);
        assert_eq!(79, base.beacons.len());
    }

    #[test]
    fn given_example_part2() {
        assert_eq!(3621, part2(INPUT));
    }

    #[test]
    fn scan_with_rotations() {
        let rotation1 = Scanner::parse("\
--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7");

        let rotation2 = Scanner::parse("\
--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0");

        assert_eq!(Some((
            Vector::new(0, 0, 0),
            &VectorTransform::new(
                Vector::new(-1, 0, 0),
                Vector::new(0, 0, -1),
                Vector::new(0, -1, 0),
            ))
        ), find_match(&rotation1, &rotation2, 6, &mut Operations::new()));
    }

    #[test]
    fn scan_every_pair() {
        let s1 = Scanner::new(HashSet::from([
            Beacon::new(0, 2, 0),
            Beacon::new(4, 1, 0),
            Beacon::new(3, 3, 0)
        ]));

        let s2 = Scanner::new(HashSet::from([
            Beacon::new(-1, -1, 0),
            Beacon::new(-5,0, 0),
            Beacon::new(-2, 1, 0),
        ]));

        assert_eq!(Vector::new(5, 2, 0), find_match(&s1, &s2, 3, &mut Operations::new()).unwrap().0);
    }
}