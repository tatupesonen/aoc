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

    fn draw_screen(&self) -> String {
        let mut output = String::from("\n");
        // In the CRT the cycles start at 1, so let's also start at 1.
        // We also need draw until 241 to get the last pixel.
        for pixel in 1..241 {
            // Shift the current pixel one left so the it lines up with our CRT.
            // Line with is 40 so we take a modulo 40 and as long as it's less than 2 we draw a pixel.
            if (self.cycles[pixel as usize] - (pixel - 1) % 40).abs() < 2 {
                output += "#"
            } else {
                output += "."
            }
            // Don't draw newline on last pixel so it matches the test output exactly.
            if pixel % 40 == 0 && pixel != 240 {
                output += "\n"
            }
        }
        output
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

    fn part_two(&self, input: &str) -> String {
        let instrs: Vec<Instruction> = input.lines().map(|e| e.into()).collect();
        let mut cpu = CPU::new();
        cpu.execute(&instrs);

        cpu.draw_screen()
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
    fn part2() {
        let eq = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(&Problem.part_two(TEST_INPUT)[1..], eq);
    }
}
