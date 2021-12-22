use std::cmp::{max, min};
use std::collections::HashSet;
use sscanf::scanf;

pub fn part1(input: &str) -> usize {
    let mut state = [[[false; 101]; 101]; 101];

    for line in input.lines() {
        let (c, x1, x2, y1, y2, z1, z2) =
            scanf!(line, "{} x={}..{},y={}..{},z={}..{}",
                String, i32, i32, i32, i32, i32, i32).unwrap();

        if x1 > 50 { continue; }
        if x2 < -50 { continue; }

        if y1 > 50 { continue; }
        if y2 < -50 { continue; }

        if z1 > 50 { continue; }
        if z2 < -50 { continue; }

        let x1 = max(x1, -50);
        let x2 = min(x2, 50);

        let y1 = max(y1, -50);
        let y2 = min(y2, 50);

        let z1 = max(z1, -50);
        let z2 = min(z2, 50);

        for x in x1..(x2+1) {
            for y in y1..(y2+1) {
                for z in z1..(z2+1) {
                    let x = (x + 50) as usize;
                    let y = (y + 50) as usize;
                    let z = (z + 50) as usize;

                    state[x][y][z] = c == "on";
                }
            }
        }

        println!("{:?}", (x1, x2, y1, y2, z1, z2));
    }

    state.iter().flatten().flatten().filter(|e| **e).count()
}

struct Point {
    x: i64, y: i64, z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point { Point { x, y, z }}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Cube {
    x1: i64, x2: i64,
    y1: i64, y2: i64,
    z1: i64, z2: i64,
}

impl Cube {
    fn new(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> Cube {
        Cube { x1, x2, y1, y2, z1, z2 }
    }

    fn get_point(&self, point_index: usize) -> Point {
        match point_index {
            0 => Point::new(self.x1, self.y1, self.z1),
            1 => Point::new(self.x1, self.y1, self.z2),
            2 => Point::new(self.x1, self.y2, self.z1),
            3 => Point::new(self.x1, self.y2, self.z2),
            4 => Point::new(self.x2, self.y1, self.z1),
            5 => Point::new(self.x2, self.y1, self.z2),
            6 => Point::new(self.x2, self.y2, self.z1),
            7 => Point::new(self.x2, self.y2, self.z2),
            _ => panic!("Unknown point index"),
        }
    }

    fn contains(&self, point: &Point) -> bool {
        point.x >= self.x1 && point.x <= self.x2
            && point.y >= self.y1 && point.y <= self.y2
            && point.z >= self.z1 && point.z <= self.z2
    }

    fn contains_cube(&self, other: &Cube) -> bool {
        for i in 0..8 {
            if !self.contains(&other.get_point(i)) {
                return false;
            }
        }
        return true;
    }

    fn volume(&self) -> u64 {
        ((self.x2 + 1 - self.x1) * (self.y2 + 1 - self.y1) * (self.z2 + 1 - self.z1)) as u64
    }
}

type Range = (i64, i64);

fn range_overlap(a: &Range, b: &Range) -> Option<Range> {
    // a:  -----------
    // b:        ----------
    //     ^     ^   ^    ^
    //     a0    b0  a1   b2

    // a:  -----------
    // b:     -----
    //     ^  ^   ^  ^
    //     a0 b0  b1 a1
    if a.1 < b.0 || b.1 < a.0 {
        return None;
    }

    Some((max(a.0, b.0), min(a.1, b.1)))
}

fn cube_overlap(a: &Cube, b: &Cube) -> Option<Cube> {
    let (x1, x2) = range_overlap(&(a.x1, a.x2), &(b.x1, b.x2))?;
    let (y1, y2) = range_overlap(&(a.y1, a.y2), &(b.y1, b.y2))?;
    let (z1, z2) = range_overlap(&(a.z1, a.z2), &(b.z1, b.z2))?;
    Some(Cube::new(x1, x2, y1, y2, z1, z2))
}

fn total_volume(cubes: &[Cube]) -> u64 {
    let mut x_map: Vec<i64> = cubes.iter()
        .map(|cube| vec![cube.x1, cube.x2 + 1])
        .flatten()
        .collect::<HashSet<i64>>().drain().collect();
    x_map.sort();

    let mut y_map: Vec<i64> = cubes.iter()
        .map(|cube| vec![cube.y1, cube.y2 + 1])
        .flatten()
        .collect::<HashSet<i64>>().drain().collect();
    y_map.sort();

    let mut z_map: Vec<i64> = cubes.iter()
        .map(|cube| vec![cube.z1, cube.z2 + 1])
        .flatten()
        .collect::<HashSet<i64>>().drain().collect();
    z_map.sort();

    let mut covered: Vec<Vec<Vec<bool>>> = Vec::new();

    for _ in x_map.iter() {
        let mut z_y_plane: Vec<Vec<bool>> = Vec::new();
        for _ in y_map.iter() {
            let mut z_line: Vec<bool> = Vec::new();
            for _ in z_map.iter() {
                z_line.push(false);
            }
            z_y_plane.push(z_line);
        }
        covered.push(z_y_plane);
    }

    for cube in cubes.iter() {
        for (x_index, x) in x_map.iter().enumerate() {
            for (y_index, y) in y_map.iter().enumerate() {
                for (z_index, z) in z_map.iter().enumerate() {
                    if cube.contains(&Point::new(*x, *y, *z)) {
                        covered[x_index][y_index][z_index] = true;
                    }
                }
            }
        }
    }

    let mut total_area = 0;

    for (x_index, (x1, x2)) in x_map.iter().zip(x_map.iter().skip(1)).enumerate() {
        for (y_index, (y1, y2)) in y_map.iter().zip(y_map.iter().skip(1)).enumerate() {
            for (z_index, (z1, z2)) in z_map.iter().zip(z_map.iter().skip(1)).enumerate() {
                if covered[x_index][y_index][z_index] {
                    total_area += ((x2 - x1) * (y2 - y1) * (z2 - z1)) as u64;
                }
            }
        }
    }

    // for y_index in 0..4 {
    //     for x_index in 0..5 {
    //         if covered[x_index][y_index][0] {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    total_area
}

pub fn part2(input: &str) -> u64 {
    let mut cubes: Vec<Cube> = Vec::new();
    let mut volume_on = 0;

    for line in input.lines().rev() {
        let (c, x1, x2, y1, y2, z1, z2) =
            scanf!(line, "{} x={}..{},y={}..{},z={}..{}",
                String, i64, i64, i64, i64, i64, i64).unwrap();

        let new_cube = Cube::new(x1, x2, y1, y2, z1, z2);

        if cubes.iter().any(|cube| cube.contains_cube(&new_cube)) {
            // The cube is entirely contained with a previous cube, so we can just ignore it.
            continue;
        }

        if c == "on" {
            let mut overlaps = Vec::new();

            for cube in cubes.iter() {
                if let Some(overlap) = cube_overlap(cube, &new_cube) {
                    overlaps.push(overlap);
                }
            }

            volume_on += new_cube.volume() - total_volume(&overlaps);
        }

        cubes.push(new_cube);
    }

    volume_on
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn given_example() {
        assert_eq!(590784, part1(INPUT));
        assert_eq!(590784, part2("\
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15"));
    }

    #[test]
    fn dev() {
        let input = "\
on x=-20..26,y=0..0,z=0..0
on x=-20..33,y=0..0,z=0..0
off x=-22..28,y=0..0,z=0..0";

        println!("{:?}", part1(input));
        println!("{:?}", part2(input));
    }

    #[test]
    fn test_volume() {
        assert_eq!(1, Cube::new(1, 1, 1, 1, 1, 1).volume());
        assert_eq!(8, Cube::new(1, 2, 1, 2, 1, 2).volume());
        assert_eq!(8, Cube::new(0, 1, 2, 3, 4, 5).volume());
    }

    #[test]
    fn test_range_overlap() {
        assert_eq!(Some((3, 5)), range_overlap(&(1, 5), &(3, 8)));
        assert_eq!(Some((2, 3)), range_overlap(&(1, 3), &(2, 8)));
        assert_eq!(Some((3, 5)), range_overlap(&(3, 5), &(1, 8)));
        assert_eq!(Some((5, 5)), range_overlap(&(0, 5), &(5, 10)));
        assert_eq!(Some((5, 5)), range_overlap(&(5, 10), &(0, 5)));
        assert_eq!(None, range_overlap(&(2, 4), &(5, 8)));
    }

    #[test]
    fn test_cube_overlap() {
        assert_eq!(None, cube_overlap(
            &Cube::new(0, 5, 0, 5, 1, 2),
            &Cube::new(7, 9, 7, 9, 1, 2)
        ));

        assert_eq!(Some(Cube::new(3, 5, 3, 5, 1, 2)),
                   cube_overlap(
                        &Cube::new(0, 5, 0, 5, 1, 2),
                        &Cube::new(3, 9, 3, 9, 1, 2)
                    ));

        assert_eq!(Some(Cube::new(3, 4, 3, 4, 1, 2)),
                   cube_overlap(
                       &Cube::new(0, 5, 0, 5, 1, 2),
                       &Cube::new(3, 4, 3, 4, 1, 2)
                   ));

        assert_eq!(Some(Cube::new(5, 5, 5, 5, 0, 0)),
                   cube_overlap(
                       &Cube::new(5, 10, 5, 10, 0, 0),
                       &Cube::new(0, 5, 0, 5, 0, 0)
                   ));
    }

    #[test]
    fn test_total_area() {
        let cube1 = Cube::new(0, 9, 0, 9, 0, 0);
        let cube2 = Cube::new(0, 5, 0, 5, 0, 0);
        assert_eq!(100, total_volume(&vec![cube1, cube2]));

        let cube1 = Cube::new(5, 9, 5, 9, 0, 0);
        let cube2 = Cube::new(1, 5, 1, 5, 0, 0);
        assert_eq!(49, total_volume(&vec![cube1, cube2]));

        let cube1 = Cube::new(1, 5, 3, 4, 0, 0);
        let cube2 = Cube::new(2, 3, 2, 3, 0, 0);
        let cube3 = Cube::new(3, 4, 1, 3, 0, 0);
        assert_eq!(15, total_volume(&vec![cube1, cube2, cube3]));
    }
}