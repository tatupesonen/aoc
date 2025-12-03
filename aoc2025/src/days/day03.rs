use crate::Solution;

fn parse_day_1(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn solve(bank: &Vec<u32>, r: usize) -> u64 {
    let mut counter = 0;
    let n = bank.len();
    let mut r = n - r;
    let mut stack: Vec<u32> = Vec::with_capacity(n);
    for digit in bank {
        while let Some(&last) = stack.last() {
            if r > 0 && last < *digit {
                stack.pop();
                r -= 1;
            } else {
                break;
            }
        }
        stack.push(*digit);
    }

    if r > 0 {
        stack.truncate(stack.len() - r)
    }
    let mut mul = 1;
    for i in (0..stack.len()).rev() {
        counter += stack[i] as u64 * mul;
        mul *= 10;
    }
    counter
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> miette::Result<String> {
        let input = parse_day_1(input);
        let solver = |x| solve(&x, 2);

        Ok(input.into_iter().map(solver).sum::<u64>().to_string())
    }

    fn part_two(&self, input: &str) -> miette::Result<String> {
        let input = parse_day_1(input);
        let solver = |x| solve(&x, 12);

        Ok(input.into_iter().map(solver).sum::<u64>().to_string())
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
