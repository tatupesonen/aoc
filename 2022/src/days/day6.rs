use crate::Solution;
use std::collections::HashSet;

const PART1_SLICE_SIZE: usize = 4;
const PART2_SLICE_SIZE: usize = 14;

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        // Collect to vec so we get `.windows()`. For some reason `Chars<_>` does not implement it.
        let output: Vec<char> = input.chars().into_iter().collect();
        let it = output.windows(PART1_SLICE_SIZE).enumerate();

        let mut found = 0;
        for (idx, slice) in it {
            let set = slice.iter().collect::<HashSet<&char>>();
            if set.len() == PART1_SLICE_SIZE {
                found = idx + PART1_SLICE_SIZE;
                break;
            }
        }

        found.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        // Collect to vec so we get `.windows()`. For some reason `Chars<_>` does not implement it.
        let output: Vec<char> = input.chars().into_iter().collect();
        let it = output.windows(PART2_SLICE_SIZE).enumerate();

        let mut found = 0;
        for (idx, slice) in it {
            let set = slice.iter().collect::<HashSet<&char>>();
            if set.len() == PART2_SLICE_SIZE {
                found = idx + PART2_SLICE_SIZE;
                break;
            }
        }

        found.to_string()
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../inputs/6/test-input.txt");

    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "11");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(TEST_INPUT), "26");
    }
}
