use std::cmp::min;
use std::collections::HashMap;
use crate::day21::Player::{Player1, Player2};

struct Die {
    // Store the next roll as 0-99 instead of 1-100 to make maths a bit simpler.
    next_roll_minus_one: i32,
    num_rolls: i32,
}

impl Die {
    fn new() -> Die {
        Die { next_roll_minus_one: 0, num_rolls: 0 }
    }

    fn roll(&mut self) -> i32 {
        let result = self.next_roll_minus_one;

        self.num_rolls += 1;
        self.next_roll_minus_one = (self.next_roll_minus_one + 1) % 100;

        result + 1
    }

    fn sum_three_rolls(&mut self) -> i32 {
        self.roll() + self.roll() + self.roll()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
struct PlayerState {
    // Store the position as 0-9 instead of 1-10 to make maths a bit simpler.
    position_minus_one: i32,
    score: i32,
}

impl PlayerState {
    fn new(position: i32) -> PlayerState {
        PlayerState {
            position_minus_one: position - 1,
            score: 0,
        }
    }

    fn take_turn(&self, rolls: i32) -> PlayerState {
        let new_position_minus_one = (self.position_minus_one + rolls) % 10;
        PlayerState {
            position_minus_one: new_position_minus_one,
            score: self.score + new_position_minus_one + 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
struct GameState {
    player1: PlayerState,
    player2: PlayerState,
}

#[derive(Copy, Clone)]
enum Player {
    Player1,
    Player2,
}

impl GameState {
    fn new(p1_pos: i32, p2_pos: i32) -> GameState {
        GameState {
            player1: PlayerState::new(p1_pos),
            player2: PlayerState::new(p2_pos),
        }
    }

    fn has_been_won(&self, required_points: i32) -> bool {
        self.player1.score >= required_points
            || self.player2.score >= required_points
    }

    fn losing_player_score(&self) -> i32 {
        min(self.player1.score, self.player2.score)
    }

    fn take_turn(&self, player: Player, rolls: i32) -> GameState {
        match player {
            Player::Player1 => {
                GameState {
                    player1: self.player1.take_turn(rolls),
                    ..*self
                }
            }
            Player::Player2 => {
                GameState {
                    player2: self.player2.take_turn(rolls),
                    ..*self
                }
            }
        }
    }
}

pub fn part1(p1_pos: i32, p2_pos: i32) -> i32 {
    let mut game = GameState::new(p1_pos, p2_pos);
    let mut die = Die::new();

    loop {
        game = game.take_turn(Player1, die.sum_three_rolls());
        if game.has_been_won(1000) { break; }

        game = game.take_turn(Player2, die.sum_three_rolls());
        if game.has_been_won(1000) { break; }
    }

    die.num_rolls * game.losing_player_score()
}

type UniverseFrequencies = HashMap<GameState, u64>;

// When a player rolls the dice three times to see how far they go...
const ROLL_FREQUENCIES: [(u8, u8); 7] = [
    (3, 1),  // they get a 3 in 1 universe (1, 1, 1).
    (4, 3),  // they get a 4 in 3 universes ((1, 1, 2), (1, 2, 1), (2, 1, 1)).
    (5, 6),  // ...
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1),
];

fn take_turn(universes: &UniverseFrequencies, player: Player) -> UniverseFrequencies {
    let mut next_universes : HashMap<GameState, u64> = HashMap::new();

    for (universe, universe_frequency) in universes.iter() {
        // Don't continue once a player has won.
        if universe.has_been_won(21) {
            *next_universes.entry(universe.clone()).or_insert(0)
                += universe_frequency;
            continue;
        }

        for (roll, roll_frequency) in &ROLL_FREQUENCIES {
            let new_state = universe.take_turn(player, *roll as i32);
            *next_universes.entry(new_state).or_insert(0)
                += universe_frequency * (*roll_frequency as u64);
        }
    }

    next_universes
}

fn all_won(universes: &UniverseFrequencies) -> bool {
    universes.iter().all(|(u, _)| u.has_been_won(21))
}

pub fn part2(p1_pos: i32, p2_pos: i32) -> (u64, u64) {
    let mut universes: UniverseFrequencies = HashMap::new();
    universes.insert(GameState::new(p1_pos, p2_pos), 1);

    loop {
        universes = take_turn(&universes, Player1);
        if all_won(&universes) { break; }

        universes = take_turn(&universes, Player2);
        if all_won(&universes) { break; }
    }

    let mut p1_wins: u64 = 0;
    let mut p2_wins: u64 = 0;

    for (universe, frequency) in universes.iter() {
        if universe.player1.score > universe.player2.score {
            p1_wins += frequency;
        } else {
            p2_wins += frequency;
        }
    }

    (p1_wins, p2_wins)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_die() {
        let mut die = Die {
            next_roll_minus_one: 97,
            num_rolls: 0,
        };

        assert_eq!(98, die.roll());
        assert_eq!(99, die.roll());
        assert_eq!(100, die.roll());
        assert_eq!(1, die.roll());

        let mut die = Die::new();
        assert_eq!(6, die.sum_three_rolls());
        assert_eq!(15, die.sum_three_rolls());
        assert_eq!(24, die.sum_three_rolls());
    }

    #[test]
    fn given_example_part1() {
        assert_eq!(739785, part1(4, 8));
    }

    #[test]
    fn challenge_part1() {
        assert_eq!(925605, part1(6, 9));
    }

    #[test]
    fn given_example_part2() {
        assert_eq!((444356092776315, 341960390180808), part2(4, 8));
    }

    #[test]
    fn challenge_part2() {
        assert_eq!((486638407378784, 413013330504401), part2(6, 9));
    }
}