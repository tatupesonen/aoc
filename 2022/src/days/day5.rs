use std::{collections::HashMap, str::FromStr};

use crate::Solution;

#[derive(Debug, Copy, Clone)]
struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse one row to an instruction
        let mut tokens = s
            .split_whitespace()
            .filter_map(|word| word.parse::<usize>().ok());
        let (amount, from, to) = (
            tokens.next().unwrap(),
            tokens.next().unwrap(),
            tokens.next().unwrap(),
        );
        Ok(Instruction { from, to, amount })
    }
}

fn parse_input(input: &str) -> (Vec<Instruction>, HashMap<usize, Vec<char>>) {
    let boxes = input.split("\n\n").collect::<Vec<&str>>()[0]
        .lines()
        .collect::<Vec<&str>>();
    let instructions: Vec<Instruction> = input.split("\n\n").collect::<Vec<&str>>()[1]
        .lines()
        .filter_map(|line| line.parse::<Instruction>().ok())
        .collect();
    let mut rows = vec![];
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    for row in boxes.into_iter() {
        let it: Vec<_> = row
            .chars()
            .into_iter()
            .enumerate()
            .filter_map(|(idx, c)| if (idx + 1) % 4 == 0 { None } else { Some(c) })
            .collect();
        let chunked: Vec<String> = it.chunks(3).map(String::from_iter).collect();
        rows.push(chunked);
    }
    rows.pop();
    // go through each row and push them to their cols
    for row in rows.into_iter() {
        row.into_iter().enumerate().for_each(|(idx, item)| {
            let stack = stacks.entry(idx + 1).or_insert(Vec::new());
            if item != "   " {
                stack.push(item.chars().collect::<Vec<char>>()[1]);
            }
        });
    }
    (instructions, stacks)
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let (instructions, mut stacks) = parse_input(input);
        // Manipulate
        for instr in instructions {
            // get target and destination stacks
            let items: Vec<char> = stacks
                .get_mut(&instr.from)
                .unwrap()
                .drain(0..instr.amount)
                .rev()
                .collect();
            let to = stacks.get_mut(&instr.to).unwrap();
            to.splice(0..0, items);
        }
        let mut output = String::new();
        // get tops of all stacks
        for col in 1..stacks.keys().len() + 1 {
            let top = stacks.get(&col).unwrap().first();
            if let Some(c) = top {
                output.push(*c)
            }
        }

        output
    }

    fn part_two(&self, input: &str) -> String {
        let (instructions, mut stacks) = parse_input(input);
        // Manipulate
        for instr in instructions {
            // get target and destination stacks
            let items: Vec<char> = stacks
                .get_mut(&instr.from)
                .unwrap()
                .drain(0..instr.amount)
                .collect();
            let to = stacks.get_mut(&instr.to).unwrap();
            to.splice(0..0, items);
        }
        let mut output = String::new();
        // get tops of all stacks
        for col in 1..stacks.keys().len() + 1 {
            let top = stacks.get(&col).unwrap().first();
            if let Some(c) = top {
                output.push(*c)
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../inputs/5/test-input.txt");

    use super::*;

    #[test]
    fn d5_part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "CMZ");
    }

    #[test]
    fn d5_part2() {
        assert_eq!(Problem.part_two(TEST_INPUT), "MCD");
    }
}
