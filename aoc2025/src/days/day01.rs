use std::ops::{Add, Sub};

use std::default::Default;

use crate::Solution;

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl Rotation {
    fn dir(&self) -> i32 {
        match *self {
            Rotation::Left(x) => -x,
            Rotation::Right(x) => x,
        }
    }
}

fn parse_day1(s: &str) -> Vec<Rotation> {
    s.lines()
        .map(|line| {
            let (first, rest) = line.split_at(1);
            let num: i32 = rest.parse().unwrap();
            match first {
                "R" => Rotation::Right(num),
                "L" => Rotation::Left(num),
                _ => panic!("Invalid rotation"),
            }
        })
        .collect()
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> miette::Result<String> {
        let rots = parse_day1(input);
        let mut dial = 50;
        let mut counter = 0;

        for rot in rots {
            match rot {
                Rotation::Left(amount) => dial = dial.sub(amount).rem_euclid(100),
                Rotation::Right(amount) => dial = dial.add(amount).rem_euclid(100),
            }
            counter += if dial == 0 { 1 } else { 0 };
        }

        Ok(counter.to_string())
    }

    fn part_two(&self, input: &str) -> miette::Result<String> {
        let rots = parse_day1(input);
        let mut dial = Dial::default();

        for rot in rots {
            dial.rotate(rot);
        }

        Ok(dial.counter.to_string())
    }

    fn day(&self) -> u16 {
        1
    }
}

#[derive(Debug)]
struct Dial {
    value: i32,
    counter: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            value: 50,
            counter: Default::default(),
        }
    }
}
impl Dial {
    fn rotate(&mut self, rot: Rotation) {
        const DIAL_SIZE: i32 = 100;
        let new_dial = self.value + rot.dir();
        let mut revolutions = (new_dial / DIAL_SIZE).abs();

        if self.value != 0 && new_dial <= 0 {
            revolutions += 1;
        }

        self.counter += revolutions;
        self.value = new_dial.rem_euclid(DIAL_SIZE);
    }
}

#[cfg(test)]
mod tests {
    use aoc2025::{input, test_input};
    use super::*;
    const TEST_INPUT: &str = test_input!("01");
    const INPUT: &str = input!("01");


    #[test]
    fn part1_test() {
        assert_eq!(Problem.part_one(TEST_INPUT).unwrap(), "3");
    }

    #[test]
    fn part2_test() {
        assert_eq!(Problem.part_two(TEST_INPUT).unwrap(), "6");
    }

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(INPUT).unwrap(), "1043");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(INPUT).unwrap(), "5963");
    }
}
