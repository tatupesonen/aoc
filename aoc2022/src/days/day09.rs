use std::collections::BTreeSet;

use crate::Solution;
type Point = (i32, i32);

pub fn chebyshev_distance(x: Point, y: Point) -> i32 {
    std::cmp::max((x.0 - y.0).abs(), (x.1 - y.1).abs())
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

    fn part_two(&self, input: &str) -> String {
        let moves = get_moves(input);

        let mut snake = [(0, 0); 10];
        let mut tail_visited: BTreeSet<(i32, i32)> = BTreeSet::from(snake);

        for mov in moves {
            move_node(mov, &mut snake[0]);
            for i in 1..snake.len() {
                let head = snake[i - 1];
                let tail = snake[i];

                if chebyshev_distance(head, tail) > 1 {
                    match head.0.cmp(&tail.0) {
                        std::cmp::Ordering::Less => snake[i].0 -= 1,
                        std::cmp::Ordering::Greater => snake[i].0 += 1,
                        _ => {}
                    }

                    match head.1.cmp(&tail.1) {
                        std::cmp::Ordering::Less => snake[i].1 -= 1,
                        std::cmp::Ordering::Greater => snake[i].1 += 1,
                        _ => {}
                    }
                }
            }
            tail_visited.insert(snake[snake.len() - 1]);
        }

        tail_visited.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/9/test-input.txt");
    const P2_INPUT: &str = include_str!("../../inputs/9/p2-input.txt");

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "13");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(P2_INPUT), "36");
    }
}
