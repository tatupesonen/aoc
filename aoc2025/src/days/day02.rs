use crate::util::*;
use crate::Solution;
use miette::{miette, Context, IntoDiagnostic, Result};
use std::ops::RangeInclusive;

fn parse(input: &str) -> Result<Vec<RangeInclusive<u64>>> {
    input
        .split(',')
        .map(|segment| -> Result<RangeInclusive<u64>> {
            let (start_str, end_str) = segment
                .split_once('-')
                .ok_or_else(|| miette!("Invalid segment '{segment}', expected -"))?;

            let start = start_str
                .parse::<u64>()
                .into_diagnostic()
                .wrap_err_with(|| format!("Unable to parse start value '{start_str}'"))?;

            let end = end_str
                .parse::<u64>()
                .into_diagnostic()
                .wrap_err_with(|| format!("Unable to parse end value '{end_str}'"))?;

            Ok(start..=end)
        })
        .collect()
}

fn double(n: u64) -> u64 {
    let s = n.to_string();
    if !s.len().half() == 0 {
        return 0;
    }

    let half = s.len().half();
    if s[half..] == s[..half] {
        n
    } else {
        0
    }
}

fn repeated(n: u64) -> u64 {
    let s = n.to_string();
    let len = s.len();

    for sub_len in 1..=len.half() {
        if !len.is_multiple_of(sub_len) {
            continue;
        }

        let part = &s[..sub_len];

        if s == part.repeat(len / sub_len) {
            return n;
        }
    }

    0
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> miette::Result<String> {
        let ranges = parse(input)?;

        let count: u64 = ranges.into_iter().flat_map(|range| range.map(double)).sum();

        Ok(count.to_string())
    }

    fn part_two(&self, input: &str) -> miette::Result<String> {
        let ranges = parse(input)?;

        let count: u64 = ranges
            .into_iter()
            .flat_map(|range| range.map(repeated))
            .sum();

        Ok(count.to_string())
    }

    fn day(&self) -> u16 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/02/test-input.txt");
    const INPUT: &str = include_str!("../../inputs/02/input.txt");

    #[test]
    fn part1_test() {
        assert_eq!(Problem.part_one(TEST_INPUT).unwrap(), "1227775554");
    }

    #[test]
    fn part2_test() {
        assert_eq!(Problem.part_two(TEST_INPUT).unwrap(), "4174379265");
    }

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(INPUT).unwrap(), "37314786486");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(INPUT).unwrap(), "47477053982");
    }
}
