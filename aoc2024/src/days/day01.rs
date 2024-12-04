use std::collections::HashMap;

use crate::Solution;

fn parse_day1(input: &str) -> miette::Result<(Vec<i32>, Vec<i32>)> {
    let mut first: Vec<i32> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .next()
                .ok_or_else(|| miette::miette!("Missing first value in line"))?
                .parse::<i32>()
                .map_err(|e| miette::miette!("Failed to parse first value as i32: {}", e))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut second: Vec<i32> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .nth(1)
                .ok_or_else(|| miette::miette!("Missing second value in line"))?
                .parse::<i32>()
                .map_err(|e| miette::miette!("Failed to parse second value as i32: {}", e))
        })
        .collect::<Result<Vec<_>, _>>()?;

    first.sort();
    second.sort();

    Ok((first, second))
}

fn distance(it: (i32, i32)) -> i32 {
    (it.0 - it.1).abs()
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> miette::Result<String> {
        let (first, second) = parse_day1(input)?;
        Ok(Iterator::zip(first.into_iter(), second)
            .map(distance)
            .sum::<i32>()
            .to_string())
    }

    fn part_two(&self, input: &str) -> miette::Result<String> {
        let (first, second) = parse_day1(input)?;
        let mut m: HashMap<i32, usize> = HashMap::new();
        for x in second {
            *m.entry(x).or_default() += 1;
        }

        Ok(first
            .iter()
            .map(|v| (*v as usize) * m.get(v).unwrap_or(&0))
            .sum::<usize>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = test_input!("01");
    const INPUT: &str = input!("01");

    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(Problem.part_one(TEST_INPUT).unwrap(), "11");
    }

    #[test]
    fn part2_test() {
        assert_eq!(Problem.part_two(TEST_INPUT).unwrap(), "31");
    }

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(INPUT).unwrap(), "2066446");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(INPUT).unwrap(), "24931009");
    }
}
