use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::util::*;
use crate::Solution;
use miette::{miette, Context, IntoDiagnostic, Result};

fn parse_day_1(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> miette::Result<String> {
        let input = parse_day_1(input);
        let mut counter: u32 = 0;

        for bank in input {
            let mut max = 0;
            let mut first_max_idx = 0;
            for n in 0..bank.len() - 1 {
                if bank[n] > max {
                    max = bank[n];
                    first_max_idx = n;
                }
            }
            let mut second_max = 0;
            for n in (first_max_idx + 1)..bank.len() {
                if bank[n] > second_max {
                    second_max = bank[n];
                }
            }
            let val = (max * 10) + second_max;
            counter += val;
        }
        Ok(counter.to_string())
    }

    fn part_two(&self, input: &str) -> miette::Result<String> {
        let input = parse_day_1(input);
        let mut counter: u64 = 0;

        for bank in input {
            let n = bank.len();
            let mut r = n - 12;
            let mut stack: Vec<u32> = Vec::with_capacity(n);
            for digit in bank {
                while let Some(&last) = stack.last() {
                    if r > 0 && last < digit {
                        stack.pop();
                        r -= 1;
                    } else {
                        break;
                    }
                }
                stack.push(digit);
            }

            if r > 0 {
                stack.truncate(stack.len() - r)
            }
            println!("{stack:?}");
            let num: String = stack.into_iter().filter_map(|d| char::from_digit(d, 10)).collect();
            counter += num.parse::<u64>().unwrap();
        }

        Ok(counter.to_string())
    }

    fn day(&self) -> u16 {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/03/test-input.txt");
    const INPUT: &str = include_str!("../../inputs/03/input.txt");

    #[test]
    fn part1_test() {
        assert_eq!(Problem.part_one(TEST_INPUT).unwrap(), "357");
    }

    #[test]
    fn part2_test() {
        assert_eq!(Problem.part_two(TEST_INPUT).unwrap(), "3121910778619");
    }

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(INPUT).unwrap(), "17324");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(INPUT).unwrap(), "171846613143331");
    }
}
