use crate::Solution;

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

type Round = (Move, Move);

impl From<i32> for Move {
    fn from(i: i32) -> Self {
        match i {
            1 => Move::Rock,
            2 => Move::Paper,
            3 => Move::Scissors,
            _ => panic!("This should not happen."),
        }
    }
}

#[derive(Debug)]
enum Goal {
    Win,
    Tie,
    Lose,
}
impl From<char> for Goal {
    fn from(c: char) -> Self {
        match c {
            'X' => Goal::Lose,
            'Y' => Goal::Tie,
            'Z' => Goal::Win,
            _ => panic!("this should not happen"),
        }
    }
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

// Paper loses to scissors
// Rock loses to paper
// Scissors loses to Rock
fn get_losing_move(mov: Move) -> Move {
    match mov {
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
        Move::Rock => Move::Scissors,
    }
}

// Scissors wins paper
// Rock wins scissors
// Paper wins rock
fn get_winning_move(mov: Move) -> Move {
    match mov {
        Move::Scissors => Move::Rock,
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
    }
}

fn play(round: Round) -> i32 {
    match round {
        (Move::Paper, Move::Paper) => round.1 as i32 + 3,
        (Move::Paper, Move::Scissors) => round.1 as i32 + 6,
        (Move::Paper, Move::Rock) => round.1 as i32,
        (Move::Rock, Move::Paper) => round.1 as i32 + 6,
        (Move::Rock, Move::Rock) => round.1 as i32 + 3,
        (Move::Rock, Move::Scissors) => round.1 as i32,
        (Move::Scissors, Move::Scissors) => round.1 as i32 + 3,
        (Move::Scissors, Move::Paper) => round.1 as i32,
        (Move::Scissors, Move::Rock) => round.1 as i32 + 6,
    }
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let moves: Vec<Round> = input
            .lines()
            .map(|line| {
                let items: Vec<Move> = line.chars().filter_map(|s| s.try_into().ok()).collect();

                (items[0], items[1])
            })
            .collect();

        let total: i32 = moves.into_iter().fold(0, |acc, round| acc + play(round));

        total.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let moves: Vec<(Move, Goal)> = input
            .lines()
            .map(|line| {
                let mut it = line.chars();
                let first: Move = it.next().unwrap().try_into().unwrap();
                let _ = it.next();
                let goal: Goal = it.next().unwrap().into();
                (first, goal)
            })
            .collect();

        let moves: Vec<(Move, Move)> = moves
            .into_iter()
            .map(|(opponent, goal)| {
                let mov = match goal {
                    Goal::Lose => get_losing_move(opponent),
                    Goal::Tie => opponent,
                    Goal::Win => get_winning_move(opponent),
                };
                (opponent, mov)
            })
            .collect();

        let total: i32 = moves.into_iter().fold(0, |acc, round| acc + play(round));

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../inputs/2/test-input.txt");

    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "15");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(TEST_INPUT), "12");
    }
}
