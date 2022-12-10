use crate::Solution;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop,
    Addx(i32),
}
impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let toks: Vec<&str> = s.split_whitespace().collect();
        match toks[0] {
            "noop" => Self::Nop,
            "addx" => Self::Addx(toks[1].parse().unwrap()),
            _ => panic!("uhh"),
        }
    }
}

struct CPU {
    cycles: Vec<i32>,
    x: i32,
}
impl CPU {
    fn new() -> Self {
        Self {
            cycles: vec![-1],
            x: 1,
        }
    }
    fn execute(&mut self, instrs: &Vec<Instruction>) {
        for i in 0..instrs.len() {
            match instrs[i] {
                Instruction::Nop => self.cycles.push(self.x),
                Instruction::Addx(x) => {
                    self.cycles.push(self.x);
                    self.cycles.push(self.x);
                    self.x += x;
                }
            }
        }
    }

    fn signal_strength(&self) -> i32 {
        (20..self.cycles.len() + 1)
            .step_by(40)
            .fold(0, |acc, i| acc + (self.cycles[i] * i as i32))
    }
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let instrs: Vec<Instruction> = input.lines().map(|e| e.into()).collect();
        let mut cpu = CPU::new();
        cpu.execute(&instrs);

        cpu.signal_strength().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/10/test-input.txt");

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "13140");
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(Problem.part_two(TEST_INPUT), "36");
    }
}
