use std::collections::VecDeque;

use crate::Solution;

#[derive(Debug)]
enum Operation {
    Mul(Option<u64>), // if using old, it's none.
    Add(u64),
}
impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        let tokens = s.split_whitespace().collect::<Vec<&str>>();
        let op = tokens[0];
        let val = tokens[1];
        match op {
            "*" => {
                if val == "old" {
                    Self::Mul(None)
                } else {
                    Self::Mul(val.parse().ok())
                }
            }
            _ => Self::Add(val.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
struct Test {
    num: u64,
    true_branch: usize,
    false_branch: usize,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    inspect_count: u64,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let lines: Vec<&str> = s.lines().collect();
        let items: VecDeque<u64> = lines[1]
            .trim()
            .split("Starting items: ")
            .collect::<Vec<&str>>()[1]
            .split(',')
            .map(|item| item.trim().parse::<u64>().unwrap())
            .collect();
        let operation: Operation = lines[2]
            .trim()
            .split("Operation: new = old ")
            .collect::<Vec<&str>>()[1]
            .into();
        let test = lines[3]
            .split_whitespace()
            .filter_map(|e| e.parse().ok())
            .collect::<Vec<u64>>()[0];
        let true_branch = lines[4]
            .split_whitespace()
            .filter_map(|e| e.parse().ok())
            .collect::<Vec<usize>>()[0];
        let false_branch = lines[5]
            .split_whitespace()
            .filter_map(|e| e.parse().ok())
            .collect::<Vec<usize>>()[0];

        Monkey {
            items,
            operation,
            inspect_count: 0,
            test: Test {
                false_branch,
                num: test,
                true_branch,
            },
        }
    }
}

impl Monkey {
    fn inspect(&mut self, should_lower: bool, remainder: u64) -> u64 {
        self.inspect_count += 1;
        let item = self.items.pop_front().unwrap();
        let level = match self.operation {
            Operation::Mul(val) => {
                // there's a number
                if let Some(x) = val {
                    let calc = item * x;
                    calc % remainder
                } else {
                    item * item
                }
            }
            Operation::Add(val) => item + val,
        };
        if should_lower {
            level / 3
        } else {
            level
        }
    }
    fn do_test(&self, item: u64) -> usize {
        match item % self.test.num == 0 {
            true => self.test.true_branch,
            false => self.test.false_branch,
        }
    }
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        // split on \n\n
        let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|e| e.into()).collect();
        // chinese remainder theorem
        let remainder = monkeys.iter().map(|e| e.test.num).product::<u64>();
        for _ in 0..20 {
            for mnk_idx in 0..monkeys.len() {
                for _ in 0..monkeys[mnk_idx].items.len() {
                    let monkey = &mut monkeys[mnk_idx];
                    let item = monkey.inspect(true, remainder);
                    let should_send_to = monkey.do_test(item);
                    monkeys[should_send_to].items.push_back(item);
                }
            }
        }
        monkeys.sort_by_key(|e| e.inspect_count);
        monkeys
            .iter()
            .rev()
            .take(2)
            .map(|mnk| mnk.inspect_count)
            .product::<u64>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        // split on \n\n
        let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|e| e.into()).collect();
        let remainder = monkeys.iter().map(|e| e.test.num).product::<u64>();
        for _ in 0..10000 {
            for mnk_idx in 0..monkeys.len() {
                for _ in 0..monkeys[mnk_idx].items.len() {
                    let monkey = &mut monkeys[mnk_idx];
                    let item = monkey.inspect(false, remainder);
                    let should_send_to = monkey.do_test(item);
                    monkeys[should_send_to].items.push_back(item);
                }
            }
        }
        monkeys.sort_by_key(|e| e.inspect_count);
        monkeys
            .iter()
            .rev()
            .take(2)
            .map(|mnk| mnk.inspect_count)
            .product::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/11/test-input.txt");

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "10605");
    }

    #[test]
    fn part2() {
        assert_eq!(&Problem.part_two(TEST_INPUT), "2713310158");
    }
}
