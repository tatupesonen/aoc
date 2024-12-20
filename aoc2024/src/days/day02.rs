use crate::Solution;

fn parse(input: &str) -> Vec<Vec<i32>> {
    let first = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|i| i.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect();
    first
}

fn is_safe(sequence: &[i32], mut dampener_levels: usize) -> bool {
    for window in sequence.windows(2) {
        let diff = window[1] - window[0];

        if diff.abs() > 3 || diff == 0 {
            if dampener_levels > 0 {
                dampener_levels -= 1;
                continue;
            }
            return false;
        }

        if diff > 0 && sequence.windows(2).any(|w| w[1] < w[0]) {
            if dampener_levels > 0 {
                dampener_levels -= 1;
                continue;
            }
            return false;
        }
    }

    true
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> miette::Result<String> {
        let reports = parse(input);

        Ok(reports
            .iter()
            .map(|levels| is_safe(levels, 0))
            .filter(|&e| e)
            .count()
            .to_string())
    }

    fn part_two(&self, input: &str) -> miette::Result<String> {
        let reports = parse(input);

        Ok(reports
            .iter()
            .map(|levels| is_safe(levels, 1))
            .filter(|&e| e)
            .count()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = test_input!("02");
    const INPUT: &str = input!("02");

    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(Problem.part_one(TEST_INPUT).unwrap(), "2");
    }

    #[test]
    fn part2_test() {
        assert_eq!(Problem.part_two(TEST_INPUT).unwrap(), "5");
    }

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(INPUT).unwrap(), "326");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(INPUT).unwrap(), "381");
    }
}
