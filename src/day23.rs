use std::cmp::{Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Formatter, Write};
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
enum Type {
    A, B, C, D,
}

impl Type {
    fn room_x(&self) -> u8 {
        match self {
            Type::A => 2,
            Type::B => 4,
            Type::C => 6,
            Type::D => 8,
        }
    }

    fn cost(&self) -> u32 {
        match self {
            Type::A => 1,
            Type::B => 10,
            Type::C => 100,
            Type::D => 1000,
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::A => f.write_char('A'),
            Type::B => f.write_char('B'),
            Type::C => f.write_char('C'),
            Type::D => f.write_char('D'),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone)]
struct Pos {
    x: u8,
    y: u8,
}

impl Pos {
    fn new(x: u8, y: u8) -> Pos {
        Pos { x, y }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    /// TODO: Try this with a simpler data structure.
    positions: HashMap<Pos, Type>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.positions.iter().for_each(|(pos, t)| {
            pos.hash(state);
            t.hash(state);
        })
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("#############\n")?;
        for y in 0..3 {
            f.write_char('#')?;
            for x in 0..11 {
                if let Some(t) = self.positions.get(&Pos::new(x, y)) {
                    t.fmt(f)?;
                } else {
                    f.write_char(' ')?;
                }
            }
            f.write_str("#\n")?;
        }
        f.write_str("#############\n")
    }
}

fn abs_diff(a: u8, b: u8) -> u8 {
    let min = u8::min(a, b);
    let max = u8::max(a, b);
    max - min
}

// TODO: Turn into a map?
const ROOM_A: u8 = 2;
const ROOM_B: u8 = 4;
const ROOM_C: u8 = 6;
const ROOM_D: u8 = 8;

impl State {
    fn new(a1: Type, a2: Type, b1: Type, b2: Type,
            c1: Type, c2: Type, d1: Type, d2: Type) -> State {
        let mut positions = HashMap::new();

        positions.insert(Pos::new(ROOM_A, 1), a1);
        positions.insert(Pos::new(ROOM_A, 2), a2);
        positions.insert(Pos::new(ROOM_B, 1), b1);
        positions.insert(Pos::new(ROOM_B, 2), b2);
        positions.insert(Pos::new(ROOM_C, 1), c1);
        positions.insert(Pos::new(ROOM_C, 2), c2);
        positions.insert(Pos::new(ROOM_D, 1), d1);
        positions.insert(Pos::new(ROOM_D, 2), d2);

        State { positions }
    }

    fn new_state(&self, old: &Pos, new: &Pos) -> State {
        let mut new_positions = self.positions.clone();

        if old != new {
            let t = new_positions.remove(old).unwrap();
            new_positions.insert(*new, t);
        }

        State {
            positions: new_positions
        }
    }

    /// Is the amphipod at its destination?
    fn at_destination(&self, pos: &Pos, room_size: u8) -> bool {
        if pos.y == 0 { return false; }

        if let Some(pod) = self.positions.get(pos) {
            if pos.x != pod.room_x() {
                return false;
            }

            // Ensure this cell and all those below are filled with the right type of pod.
            for y in pos.y..(room_size + 1) {
                if self.positions.get(&Pos::new(pos.x, y)) != Some(pod) {
                    return false;
                }
            }

            return true;
        }

        return false;
    }

    fn is_finished(&self) -> bool {
        self.positions.iter().all(|(pos, t)| pos.x == t.room_x())
    }

    /// Checks whether a pod can move into that room. It will return None if the room contains a
    /// pod that should not end up there. If the room can be moved into, it returns the y value
    /// the pod will end up at.
    fn room_open(&self, t: &Type, room_size: u8) -> Option<u8> {
        for i in 0..room_size {
            let y = room_size - i;
            let pos = Pos::new(t.room_x(), y);

            // Find the lowest empty room.
            if let Some(pod) = self.positions.get(&pos) {
                if pod != t {
                    // The cell is occupied by a pod of the wrong type. We can't move into this
                    // room at all.
                    return None;
                }
                // else check the next cell up.
            } else {
                // We found an empty spot!
                return Some(y);
            }
        }

        panic!("Room should not be full");
    }

    /// Returns whether the pod at given coordinates can leave the room.
    fn can_leave(&self, pos: &Pos) -> bool {
        for y in 0..pos.y {
            if self.positions.contains_key(&Pos::new(pos.x, y)) {
                return false;
            }
        }
        return true;
    }

    /// Returns possible states resulting from the pod at the given position moving.
    fn possible_moves(&self, pos: &Pos, room_size: u8) -> Vec<(u32, State)> {
        let mut possible_states: Vec<(u32, State)> = Vec::new();

        if self.at_destination(pos, room_size) { return possible_states; }

        let pod = self.positions.get(pos).unwrap();

        if pos.y == 0 {
            // The pod can only move into its destination.
            if let Some(y) = self.room_open(pod, room_size) {
                // Check the path between pos.x and room.x is clear.
                if let Some(x_dist) = self.clear_path(pos.x, pod.room_x()) {
                    let dest = Pos::new(pod.room_x(), y);

                    let cost = ((y + x_dist) as u32) * pod.cost();

                    possible_states.push((cost, self.new_state(pos, &dest)));
                }
            }
        } else {
            if self.can_leave(pos) {
                for x in [0, 1, 3, 5, 7, 9, 10] {
                    // Check nothing is already occupying that spot.
                    if self.positions.contains_key(&Pos::new(x, 0)) { continue; }

                    // Check there's a clear path to that spot.
                    if let Some(x_dist) = self.clear_path(pos.x, x) {
                        let dest = Pos::new(x, 0);
                        let cost = ((pos.y + x_dist) as u32) * pod.cost();

                        possible_states.push((cost, self.new_state(pos, &dest)));
                    }
                }
            }
        }
        possible_states
    }

    fn all_possible_moves(&self, room_size: u8) -> Vec<(u32, State)> {
        self.positions.iter()
            .map(|(pos, _)| self.possible_moves(pos, room_size))
            .flatten()
            .collect()
    }

    /// Checks if there is a clear path along the top corridor between the two
    /// x positions. Does not check the end and start position. If there is, it returns the
    /// distance.
    fn clear_path(&self, x1: u8, x2: u8) -> Option<u8> {
        // TODO: I'm doing a lot of duplicate work building the possible paths.

        let x_min = u8::min(x1, x2);
        let x_max = u8::max(x1, x2);

        // TODO: Don't bother checking positions 2, 4, 6, 8.
        for x in (x_min + 1)..x_max {
            if self.positions.contains_key(&Pos::new(x, 0)) {
                return None;
            }
        }
        return Some(x_max - x_min);
    }

    /// Gives a lower bound on the cost between the current state and the finished state.
    fn distance_estimate(&self) -> u32 {
        self.positions.iter()
            .map(|(pos, t)| {
                // abs_diff(pos.x, t.room_x()) as u32 * t.cost()
                if pos.x == t.room_x() {
                    0
                } else {
                    (abs_diff(pos.x, t.room_x()) + pos.y + 1) as u32 * t.cost()
                }
            })
            .sum()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SearchState {
    cost: u32,
    estimate: u32,
    state: State,
}

impl PartialOrd<Self> for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.estimate).cmp(&(self.cost + self.estimate))
    }
}

fn search(state: &State, room_size: u8) {
    let mut costs: HashMap<State, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();

    costs.insert(state.clone(), 0);
    heap.push(SearchState {
        cost: 0,
        state: state.clone(),
        estimate: state.distance_estimate(),
    });

    let mut iter = 0;

    while let Some(SearchState { cost, state, .. }) = heap.pop() {
        iter += 1;
        // println!("Considering, cost: {}", cost);
        // println!("{:?}", state);
        if state.is_finished() {
            println!("Found our goal: {}, {}", cost, iter);
            return;
        }

        // We've already found a better way.
        if cost > *costs.get(&state).unwrap_or(&u32::MAX) { continue; }

        // For each new state we can get to...
        for (travel_cost, new_state) in state.all_possible_moves(room_size) {
            let new_cost = cost + travel_cost;
            // let tentative_cost = cost + travel_cost;
            // let new_cost = cost + travel_cost + new_state.distance_estimate();

            // If we've found a better cost to a destination, add it
            // (or if we've found a new destination).
            if new_cost < *costs.get(&new_state).unwrap_or(&u32::MAX) {
                costs.insert(new_state.clone(), new_cost);

                let next = SearchState {
                    cost: new_cost,
                    estimate: new_state.distance_estimate(),
                    state: new_state,
                };

                heap.push(next);
            }
        }
    }
}

pub fn part1() {
    use crate::day23::Type::{A, B, C, D};

    let state = State::new(C, C, A, A, B, D, D, B);
    search(&state, 2);
}

pub fn part2() {
    use crate::day23::Type::{A, B, C, D};

    // #############
    // #...........#
    // ###C#A#B#D###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #C#A#D#B#
    //   #########

    let state = State {
        positions: HashMap::from([
            (Pos::new(2, 1), C),
            (Pos::new(2, 2), D),
            (Pos::new(2, 3), D),
            (Pos::new(2, 4), C),

            (Pos::new(4, 1), A),
            (Pos::new(4, 2), C),
            (Pos::new(4, 3), B),
            (Pos::new(4, 4), A),

            (Pos::new(6, 1), B),
            (Pos::new(6, 2), B),
            (Pos::new(6, 3), A),
            (Pos::new(6, 4), D),

            (Pos::new(8, 1), D),
            (Pos::new(8, 2), A),
            (Pos::new(8, 3), C),
            (Pos::new(8, 4), B),
        ])
    };

    // #############
    // #...........#
    // ###B#C#B#D###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #A#D#C#A#
    //   #########

    // let state = State {
    //     positions: HashMap::from([
    //         (Pos::new(2, 1), B),
    //         (Pos::new(2, 2), D),
    //         (Pos::new(2, 3), D),
    //         (Pos::new(2, 4), A),
    //
    //         (Pos::new(4, 1), C),
    //         (Pos::new(4, 2), C),
    //         (Pos::new(4, 3), B),
    //         (Pos::new(4, 4), D),
    //
    //         (Pos::new(6, 1), B),
    //         (Pos::new(6, 2), B),
    //         (Pos::new(6, 3), A),
    //         (Pos::new(6, 4), C),
    //
    //         (Pos::new(8, 1), D),
    //         (Pos::new(8, 2), A),
    //         (Pos::new(8, 3), C),
    //         (Pos::new(8, 4), A),
    //     ])
    // };

    search(&state, 4);
}

#[cfg(test)]
mod test {
    use crate::day23::Type::{A, B, C, D};
    use super::*;

    #[test]
    fn test_at_destination() {
        // #############
        // #...........#
        // ###B#D#C#A###
        //   #A#D#C#B#
        //   #########
        let state = State::new(B, A, D, D, C, C, B, A);

        assert_eq!(false, state.at_destination(&Pos::new(2, 1), 2));
        assert_eq!(true, state.at_destination(&Pos::new(2, 2), 2));
        assert_eq!(false, state.at_destination(&Pos::new(4, 1), 2));
        assert_eq!(false, state.at_destination(&Pos::new(4, 2), 2));
        assert_eq!(true, state.at_destination(&Pos::new(6, 1), 2));
        assert_eq!(true, state.at_destination(&Pos::new(6, 2), 2));
        assert_eq!(false, state.at_destination(&Pos::new(8, 1), 2));
        assert_eq!(false, state.at_destination(&Pos::new(8, 2), 2));
    }

    #[test]
    fn test_room_open() {
        // #############
        // #           #
        // ### # # #A###
        //   #B# #C#A#
        //   #########
        let state = State {
            positions: HashMap::from([
                (Pos::new(2, 2), B),
                (Pos::new(6, 2), C),
                (Pos::new(8, 1), A),
                (Pos::new(8, 2), A),
            ])
        };

        assert_eq!(None, state.room_open(&A, 2));
        assert_eq!(Some(2), state.room_open(&B, 2));
        assert_eq!(Some(1), state.room_open(&C, 2));
        assert_eq!(None, state.room_open(&D, 2));
    }

    #[test]
    fn test_clear_path() {
        // #############
        // #  C        #
        // ### # # # ###
        //      ...
        let state = State {
            positions: HashMap::from([
                (Pos::new(2, 0), C),
            ])
        };

        assert_eq!(None, state.clear_path(0, 3));
        assert_eq!(None, state.clear_path(1, 4));
        assert_eq!(Some(3), state.clear_path(2, 5));
    }

    #[test]
    fn test_can_leave() {
        // #############
        // #           #
        // ### # # #A###
        //   # # #C#A#
        //   #########
        let state = State {
            positions: HashMap::from([
                (Pos::new(6, 2), C),
                (Pos::new(8, 1), A),
                (Pos::new(8, 2), A),
            ])
        };

        assert_eq!(true, state.can_leave(&Pos::new(6, 2)));
        assert_eq!(true, state.can_leave(&Pos::new(8, 1)));
        assert_eq!(false, state.can_leave(&Pos::new(8, 2)));
    }

    #[test]
    fn test_possible_moves_out() {
        // #############
        // #C      A  B#
        // ### # # # ###
        //   # # #C# #
        //   #########
        let state = State {
            positions: HashMap::from([
                (Pos::new(6, 2), C),
                (Pos::new(0, 0), C),
                (Pos::new(7, 0), A),
                (Pos::new(10, 0), B),
            ])
        };

        let moves = state.possible_moves(&Pos::new(0, 0), 2);
        assert_eq!(1, moves.len());
        assert_eq!(700, moves[0].0);

        let moves = state.possible_moves(&Pos::new(7, 0), 2);
        assert_eq!(1, moves.len());
        assert_eq!(7, moves[0].0);

        assert_eq!(0, state.possible_moves(&Pos::new(10, 0), 2).len());
    }

    #[test]
    fn test_possible_moves_in() {
        // #############
        // #     D     #
        // ###B# # # ###
        //   #B# #C# #
        //   #########
        let state = State {
            positions: HashMap::from([
                (Pos::new(2, 1), B),
                (Pos::new(2, 2), B),
                (Pos::new(5, 0), D),
                (Pos::new(6, 2), C),
            ])
        };

        // C is already in its final position.
        assert_eq!(0, state.possible_moves(&Pos::new(6, 2), 2).len());

        // The bottom B can't get past the top one.
        assert_eq!(0, state.possible_moves(&Pos::new(2, 2), 2).len());

        // The top B can go to 3 different spots.
        assert_eq!(3, state.possible_moves(&Pos::new(2, 1), 2).len());
    }

    #[test]
    fn given_example_part1() {
        // #############
        // #...........#
        // ###B#C#B#D###
        //   #A#D#C#A#
        //   #########
        let state = State::new(B, A, C, D, B, C, D, A);

        search(&state, 2);
    }

    #[test]
    fn part1() {
        let state = State::new(C, C, A, A, B, D, D, B);
        search(&state, 2);
    }

    #[test]
    fn simple() {
        // #############
        // #           #
        // ### # # # ###
        //   #C#D#A#B#
        //   #########
        let state = State {
            positions: HashMap::from([
                (Pos::new(2, 2), C),  // none, h1 (just x), h2 (x and y), h3 (* cost)
                (Pos::new(6, 2), A),  // 72, 70
                (Pos::new(4, 2), D),  // 1363, 1519, 69
                (Pos::new(8, 2), B),  // 22755, 24216, 23980, 469, 7324
            ])
        };
        search(&state, 2);
    }
}