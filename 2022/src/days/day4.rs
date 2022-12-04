use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

use crate::Solution;

#[derive(Debug)]
struct Pair {
    left: RangeInclusive<u32>,
    right: RangeInclusive<u32>,
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Get both sides
        let split_at = s.find(',').unwrap();
        let (left, right) = (&s[0..split_at], &s[(split_at + 1)..s.len()]);
        let mut l_it = left.split('-').map(|e| e.parse::<u32>().unwrap());
        let mut r_it = right.split('-').map(|e| e.parse::<u32>().unwrap());
        let (l_start, l_end) = (l_it.next().unwrap(), l_it.next().unwrap());
        let (r_start, r_end) = (r_it.next().unwrap(), r_it.next().unwrap());

        Ok(Self {
            left: l_start..=l_end,
            right: r_start..=r_end,
        })
    }
}

impl Pair {
    fn contains_other(&self) -> bool {
        self.left
            .clone()
            .into_iter()
            .all(|e| self.right.contains(&e))
            || self
                .right
                .clone()
                .into_iter()
                .all(|e| self.left.contains(&e))
    }

    fn overlaps(&self) -> bool {
        self.right
            .clone()
            .into_iter()
            .any(|e| self.left.contains(&e))
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // get max of ends
        let (end_left, end_right) = (self.left.end(), self.right.end());
        let (start_left, start_right) = (self.left.start(), self.right.start());
        let max = std::cmp::max(end_left, end_right);
        let min = std::cmp::min(start_left, start_right);

        // Write for left side
        for n in (min - 1)..=(max + 1) {
            if self.left.contains(&n) {
                write!(f, "{}", n)?;
            } else {
                write!(f, ".")?;
            }
        }
        writeln!(f)?;
        // Write for right side
        for n in (min - 1)..=(max + 1) {
            if self.right.contains(&n) {
                write!(f, "{}", n)?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let result: usize = input
            .lines()
            .filter_map(|e| e.parse::<Pair>().ok())
            .filter(|e| e.contains_other())
            .count();
        result.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let result: usize = input
            .lines()
            .filter_map(|e| e.parse::<Pair>().ok())
            .filter(|e| e.overlaps())
            .count();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../inputs/4/test-input.txt");

    use super::*;

    #[test]
    fn d4_part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "2");
    }

    #[test]
    fn d4_part2() {
        assert_eq!(Problem.part_two(TEST_INPUT), "4");
    }
}
