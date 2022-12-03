use std::collections::HashSet;

use crate::Solution;
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let compartments: usize = input
            .lines()
            .map(|e| {
                let split_at = e.len() / 2;
                e.split_at(split_at)
            })
            .map(|e| {
                // Get the longer of the two
                let (left, right) = e;
                left.chars()
                    .filter(|e| right.chars().any(|c| c == *e))
                    .take(1)
                    .map(|e| ALPHABET.chars().position(|c| c == e).unwrap() + 1)
                    .collect::<Vec<usize>>()[0]
            })
            .sum();
        compartments.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let chunks: Vec<&[&str]> = lines.chunks(3).collect();
        let total: usize = chunks
            .into_iter()
            .map(|chunk| {
                let maps = chunk
                    .iter()
                    .map(|e| {
                        e.chars()
                            .map(|e| e.to_string())
                            .collect::<HashSet<String>>()
                    })
                    .collect::<Vec<HashSet<String>>>();
                let thing = maps[1..]
                    .iter()
                    .fold(maps[0].clone(), |acc, el| {
                        let thing = el.intersection(&acc).cloned().collect();
                        thing
                    })
                    .into_iter()
                    .map(|e| ALPHABET.chars().position(|c| c.to_string() == e).unwrap() + 1)
                    .collect::<Vec<usize>>()[0];
                thing
            })
            .sum();
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../inputs/3/test-input.txt");

    use super::*;

    #[test]
    fn d3_part1() {
        assert_eq!(Problem::part_one(TEST_INPUT), "157");
    }

    #[test]
    #[ignore]
    fn d3_part2() {
        assert_eq!(Problem::part_two(TEST_INPUT), "70");
    }
}
