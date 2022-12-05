use crate::Solution;

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let output = input
            .split("\n\n")
            .map(|e| {
                e.split('\n')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .max()
            .unwrap();
        output.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut vec: Vec<u32> = input
            .split("\n\n")
            .map(|e| {
                e.split('\n')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .collect();
        vec.sort_by(|a, b| b.cmp(a));
        let output: u32 = vec.into_iter().take(3).sum();
        output.to_string()
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../inputs/1/test-input.txt");

    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "24000");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(TEST_INPUT), "45000");
    }
}