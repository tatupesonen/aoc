use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use crate::Solution;

#[derive(Debug, Clone)]
enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[derive(PartialEq, Eq)]
enum Process {
    DontProcess,
    DoProcess,
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let (_, instrs) = parse(input).unwrap();

        instrs
            .iter()
            .map(|instr| match instr {
                Instruction::Mul(x, y) => x * y,
                _ => 0,
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (_input, instructions) = parse(input).unwrap();

        instructions
            .iter()
            .fold((Process::DoProcess, 0), |(process, acc), ins| match ins {
                Instruction::Mul(a, b) => {
                    if process == Process::DoProcess {
                        (process, acc + a * b)
                    } else {
                        (process, acc)
                    }
                }
                Instruction::Do => (Process::DoProcess, acc),
                Instruction::Dont => (Process::DontProcess, acc),
            })
            .1
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../inputs/03/test-input.txt");
    const TEST_INPUT_2: &str = include_str!("../../inputs/03/test-input-2.txt");
    const INPUT: &str = include_str!("../../inputs/03/input.txt");

    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(Problem.part_one(TEST_INPUT), "161");
    }

    #[test]
    fn part2_test() {
        assert_eq!(Problem.part_two(TEST_INPUT_2), "48");
    }

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(INPUT), "178886550");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(INPUT), "87163705");
    }
}
