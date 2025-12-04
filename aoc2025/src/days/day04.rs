use std::fmt::Display;
use std::str::FromStr;

use crate::Solution;

#[derive(Debug, Clone, PartialEq)]
enum Position {
    Empty,
    Roll,
}

#[derive(Debug)]
struct Grid {
    contained: Vec<Vec<Position>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in &self.contained {
            for x in y {
                match x {
                    Position::Empty => write!(f, ".")?,
                    Position::Roll => write!(f, "@")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn position(&self, pos: (usize, usize)) -> Option<&Position> {
        self.contained.get(pos.1)?.get(pos.0)
    }

    fn map(&self) -> &Vec<Vec<Position>> {
        &self.contained
    }

    fn set(&mut self, pos: (usize, usize), t: Position) {
        self.contained[pos.1][pos.0] = t;
    }

    fn surrounding(&self, pos: (usize, usize)) -> Vec<&Position> {
        const DIRS: [(isize, isize); 8] = [
            (0, -1),  // up
            (0, 1),   // down
            (-1, 0),  // left
            (1, 0),   // right
            (-1, -1), // top-left
            (1, -1),  // top-right
            (-1, 1),  // bottom-left
            (1, 1),   // bottom-right
        ];

        let (x, y) = pos;

        DIRS.iter()
            .filter_map(|(dx, dy)| {
                let new_x = x.checked_add_signed(*dx)?;
                let new_y = y.checked_add_signed(*dy)?;
                self.position((new_x, new_y))
            })
            .collect()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let grid: Vec<Vec<Position>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c == '@' {
                            Position::Roll
                        } else {
                            Position::Empty
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(Grid { contained: grid })
    }
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> miette::Result<String> {
        let grid = Grid::from_str(input).unwrap();
        let mut counter = 0;

        for (y, row) in grid.map().iter().enumerate() {
            for (x, pos) in row.iter().enumerate() {
                if *pos == Position::Empty {
                    continue;
                }

                let surrounding = grid
                    .surrounding((x, y))
                    .iter()
                    .filter(|&&p| *p == Position::Roll)
                    .count();

                if surrounding < 4 {
                    counter += 1;
                }
            }
        }

        Ok(counter.to_string())
    }

    fn part_two(&self, input: &str) -> miette::Result<String> {
        let mut grid = Grid::from_str(input).unwrap();
        let mut counter = 0;

        loop {
            let mut to_clear = vec![];

            for (y, row) in grid.map().iter().enumerate() {
                for (x, pos) in row.iter().enumerate() {
                    if *pos == Position::Empty {
                        continue;
                    }

                    let surrounding = grid
                        .surrounding((x, y))
                        .iter()
                        .filter(|&&p| *p == Position::Roll)
                        .count();

                    if surrounding < 4 {
                        to_clear.push((x, y));
                    }
                }
            }

            if to_clear.is_empty() {
                break;
            }

            for (x, y) in to_clear {
                grid.set((x, y), Position::Empty);
                counter += 1;
            }

            print!("{}\n\n", grid);
        }

        Ok(counter.to_string())
    }

    fn day(&self) -> u16 {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/04/test-input.txt");
    const INPUT: &str = include_str!("../../inputs/04/input.txt");

    #[test]
    fn part1_test() {
        assert_eq!(Problem.part_one(TEST_INPUT).unwrap(), "13");
    }

    #[test]
    fn part2_test() {
        assert_eq!(Problem.part_two(TEST_INPUT).unwrap(), "43");
    }

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(INPUT).unwrap(), "1502");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(INPUT).unwrap(), "9083");
    }
}
