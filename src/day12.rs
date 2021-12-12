use std::collections::{HashMap, HashSet};
use crate::day12::Node::{BigCave, SmallCave};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
enum Node {
    Start,
    End,
    BigCave(char, char),
    SmallCave(char, char),
}

impl Node {
    fn parse(s: &str) -> Node {
        if s == "start" {
            return Node::Start;
        } else if s == "end" {
            return Node::End
        }

        let mut chars = s.chars();
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap_or('\0');
        if c1.is_uppercase() {
            BigCave(c1, c2)
        } else {
            SmallCave(c1, c2)
        }
    }
}

type Edges = HashMap<Node, Vec<Node>>;
type Visits = HashSet<Node>;

fn parse(input: &str) -> Edges {
    let mut edges : Edges = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split('-');
        let a = Node::parse(parts.next().unwrap());
        let b = Node::parse(parts.next().unwrap());

        if !edges.contains_key(&a) {
            edges.insert(a, Vec::new());
        }

        if !edges.contains_key(&b) {
            edges.insert(b, Vec::new());
        }

        edges.get_mut(&a).unwrap().push(b);
        edges.get_mut(&b).unwrap().push(a);
    }

    edges
}

fn search(current: &Node, edges: &Edges, visited: &Visits, can_revisit: bool) -> i32 {
    let neighbours = edges.get(current).unwrap();

    if let Node::End = current {
        println!("Got to the end");
    }

    let mut num_paths = 0;

    for neighbour in neighbours {
        let mut new_visited = visited.clone();
        let mut new_can_revisit = can_revisit;

        match neighbour {
            Node::Start => continue,
            Node::End => {
                num_paths += 1;
                continue;
            },
            Node::SmallCave(_, _) => {
                if visited.contains(neighbour) {
                    if can_revisit {
                        new_can_revisit = false;
                    } else {
                        continue;
                    }
                }

                new_visited.insert(*neighbour);
            }
            _ => {}
        }
        num_paths += search(neighbour, edges, &new_visited, new_can_revisit);
    }

    num_paths
}

pub fn part1(input: &str) -> i32 {
    let edges = parse(&input);
    search(&Node::Start, &edges, &HashSet::new(), false)
}

pub fn part2(input: &str) -> i32 {
    let edges = parse(&input);
    search(&Node::Start, &edges, &HashSet::new(), true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dev() {
        let input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        part2(input);
    }

    #[test]
    fn given_example1() {
        let input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        assert_eq!(10, part1(input));
        assert_eq!(36, part2(input));
    }

    #[test]
    fn part1_given_example2() {
        let input = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

        assert_eq!(19, part1(input));
        assert_eq!(103, part2(input));
    }

    #[test]
    fn part1_given_example3() {
        let input = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        assert_eq!(226, part1(input));
        assert_eq!(3509, part2(input));
    }
}