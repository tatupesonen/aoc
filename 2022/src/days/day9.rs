use std::collections::BTreeSet;

use crate::Solution;
type Point = (i32, i32);

pub fn chebyshev_distance(x: Point, y: Point) -> i32 {
    std::cmp::max((x.0 - y.0).abs(), (x.1 - y.1).abs())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coord(i32, i32);
impl From<Coord> for (i32, i32) {
    fn from(v: Coord) -> Self {
        (v.0, v.1)
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("wtf"),
        }
    }
}

fn get_moves(s: &str) -> Vec<Direction> {
    s.lines()
        .flat_map(|e| {
            let vals: Vec<&str> = e.split_whitespace().collect();
            let dir: Direction = vals[0].into();
            let size: usize = vals[1].parse().unwrap();
            vec![dir; size]
        })
        .collect()
}

fn move_node(m: Direction, n: &mut (i32, i32)) {
    match m {
        Direction::Up => n.1 += 1,
        Direction::Down => n.1 -= 1,
        Direction::Left => n.0 -= 1,
        Direction::Right => n.0 += 1,
    }
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let moves = get_moves(input);

        let mut head = (0, 0);
        let mut tail = (0, 0);
        let mut tail_pos: BTreeSet<(i32, i32)> = BTreeSet::from([tail]);

        for mov in moves {
            let prev_head = head;
            move_node(mov, &mut head);

            if chebyshev_distance(head, tail) > 1 {
                tail = prev_head;
                tail_pos.insert(prev_head);
            }
        }

        tail_pos.len().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/9/test-input.txt");

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "13");
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(Problem.part_two(TEST_INPUT), "8");
    }
}
