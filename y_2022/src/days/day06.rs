use crate::Solution;
use std::collections::HashSet;

const PART1_SLICE_SIZE: usize = 4;
const PART2_SLICE_SIZE: usize = 14;

fn find_marker(it: Vec<char>, slice_size: usize) -> usize {
    let it = it.windows(slice_size).enumerate();
    let mut found = 0;
    for (idx, slice) in it {
        let set = slice.iter().collect::<HashSet<&char>>();
        if set.len() == slice_size {
            found = idx + slice_size;
            break;
        }
    }
    found
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        // Collect to vec so we get `.windows()`. For some reason `Chars<_>` does not implement it.
        let output: Vec<char> = input.chars().into_iter().collect();
        let found = find_marker(output, PART1_SLICE_SIZE);
        found.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let output: Vec<char> = input.chars().into_iter().collect();
        let found = find_marker(output, PART2_SLICE_SIZE);
        found.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn part1(#[case] input: &str, #[case] expected: usize) {
        let input: Vec<char> = input.chars().into_iter().collect();
        assert_eq!(find_marker(input, PART1_SLICE_SIZE), expected);
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlh", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn part2(#[case] input: &str, #[case] expected: usize) {
        let input: Vec<char> = input.chars().into_iter().collect();
        assert_eq!(find_marker(input, PART2_SLICE_SIZE), expected);
    }
}
