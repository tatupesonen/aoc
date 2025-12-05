use crate::Solution;

struct Ingredient {
    fresh: bool,
    number: usize,
}
fn parse(s: &str) -> (Vec<(usize, usize)>, Vec<Ingredient>) {
    let (top, bottom) = s.split_once("\n\n").unwrap();

    let ranges: Vec<(usize, usize)> = top
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .collect();

    let ingredients: Vec<Ingredient> = bottom
        .lines()
        .map(|line| Ingredient {
            number: line.parse::<usize>().unwrap(),
            fresh: false,
        })
        .collect();

    (ranges, ingredients)
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> miette::Result<String> {
        let (ranges, mut ingredients) = parse(input);

        for ingredient in ingredients.iter_mut() {
            for (start, end) in &ranges {
                if ingredient.number >= *start && ingredient.number <= *end {
                    ingredient.fresh = true;
                    println!(
                        "Set {} to fresh, was in range {start}..={end}",
                        ingredient.number
                    );
                    break;
                }
            }
        }

        let count: usize = ingredients.iter().filter(|x| x.fresh).count();

        Ok(count.to_string())
    }

    fn part_two(&self, input: &str) -> miette::Result<String> {
        let (ranges, _) = parse(input);

        let mut sorted_ranges = ranges.clone();
        sorted_ranges.sort_by_key(|r| r.0);

        let mut merged_ranges: Vec<(usize, usize)> = Vec::new();
        for (start, end) in sorted_ranges {
            if let Some(last) = merged_ranges.last_mut() {
                if last.1 >= start - 1 {
                    last.1 = last.1.max(end);
                } else {
                    merged_ranges.push((start, end));
                }
            } else {
                merged_ranges.push((start, end));
            }
        }

        let total: usize = merged_ranges
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum();
        Ok(total.to_string())
    }

    fn day(&self) -> u16 {
        5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/05/test-input.txt");
    const INPUT: &str = include_str!("../../inputs/05/input.txt");

    #[test]
    fn part1_test() {
        assert_eq!(Problem.part_one(TEST_INPUT).unwrap(), "3");
    }

    #[test]
    fn part2_test() {
        assert_eq!(Problem.part_two(TEST_INPUT).unwrap(), "14");
    }

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(INPUT).unwrap(), "744");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(INPUT).unwrap(), "347468726696961");
    }
}
