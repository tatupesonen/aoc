use std::collections::HashMap;

use crate::Solution;

fn parse_day1(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();

    let mut second = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();
    first.sort();
    second.sort();

    (first, second)
}

fn distance(it: (i32, i32)) -> i32 {
    (it.0 - it.1).abs()
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let (first, second) = parse_day1(input);
        Iterator::zip(first.into_iter(), second)
            .map(distance)
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (first, second) = parse_day1(input);
        let mut m: HashMap<i32, usize> = HashMap::new();
        for x in second {
            *m.entry(x).or_default() += 1;
        }

        first
            .iter()
            .map(|v| (*v as usize) * m.get(v).unwrap_or(&0))
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../inputs/1/test-input.txt");
    const INPUT: &str = include_str!("../../inputs/1/input.txt");

    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(Problem.part_one(TEST_INPUT), "11");
    }

    #[test]
    fn part2_test() {
        assert_eq!(Problem.part_two(TEST_INPUT), "31");
    }

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(INPUT), "2066446");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(INPUT), "24931009");
    }
}
