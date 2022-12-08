use std::fmt::Display;

use crate::Solution;

pub fn solve_p1(vec: &Vec<Vec<i8>>, counts: &mut Vec<Vec<i8>>) {
    for (row_num, row) in vec.iter().enumerate() {
        let mut prev = -1;
        for (col_num, tree) in row.iter().enumerate() {
            match *tree > prev {
                true => {
                    prev = *tree;
                    counts[row_num][col_num] += 1;
                }
                _ => {}
            }
        }
    }

    for (y, row) in vec.iter().enumerate() {
        let mut prev = -1;
        for x in 0..row.len() {
            match vec[x][y] > prev {
                true => {
                    prev = vec[x][y];
                    counts[x][y] += 1;
                }
                _ => {}
            }
        }
    }

    for (y, row) in vec.iter().enumerate() {
        let mut prev = -1;
        for idx in (0..row.len()).rev() {
            match vec[y][idx] > prev {
                true => {
                    prev = vec[y][idx];
                    counts[y][idx] += 1;
                }
                _ => {}
            }
        }
    }

    for y in (0..vec.len()).rev() {
        let mut prev = -1;
        for x in (0..vec[y].len()).rev() {
            match vec[x][y] > prev {
                true => {
                    prev = vec[x][y];
                    counts[x][y] += 1;
                }
                _ => {}
            }
        }
    }
}

pub fn solve_p2(vec: &Vec<Vec<i8>>, counts: &mut Vec<Vec<u32>>) {
    for row in 0..vec.len() {
        for col in 0..vec[row].len() {
            let height = vec[row][col];

            let mut left: u32 = 0;
            let mut right: u32 = 0;
            let mut up: u32 = 0;
            let mut down: u32 = 0;

            // up
            for y in (0..row).rev() {
                up += 1;
                if vec[y][col] >= height {
                    break;
                }
            }

            // to the left of the tree
            for x in (0..col).rev() {
                left += 1;
                if vec[row][x] >= height {
                    break;
                }
            }

            // to right of the tree
            for x in (col + 1)..vec[row].len() {
                right += 1;
                if vec[row][x] >= height {
                    break;
                }
            }

            // down
            for y in (row + 1)..vec.len() {
                down += 1;
                if vec[y][col] >= height {
                    break;
                }
            }

            counts[row][col] = left * right * up * down
        }
    }
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let map: Vec<Vec<i8>> = input
            .lines()
            .map(|l| l.chars().map(|e| e.to_digit(10).unwrap() as i8).collect())
            .collect();

        let height = map.len();
        let width = map[0].len();

        let mut counts: Vec<Vec<i8>> = vec![vec![0; width]; height];
        solve_p1(&map, &mut counts);
        let count: u32 = counts
            .into_iter()
            .flatten()
            .filter_map(|e| if e > 0 { Some(1) } else { None })
            .sum();

        count.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let map: Vec<Vec<i8>> = input
            .lines()
            .map(|l| l.chars().map(|e| e.to_digit(10).unwrap() as i8).collect())
            .collect();

        let height = map.len();
        let width = map[0].len();

        let mut counts: Vec<Vec<u32>> = vec![vec![0; width]; height];
        solve_p2(&map, &mut counts);
        let count: u32 = counts.into_iter().flatten().max().unwrap();

        count.to_string()
    }
}
// 01234
// |||||
// 30373 - 0
// 25512 - 1
// 65332 - 2
// 33549 - 3
// 35390 - 4

// fn print_vec<T: Display>(v: &Vec<Vec<T>>) {
//     for row in v {
//         for col in row {
//             print!("{}", col);
//         }
//         println!();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/8/test-input.txt");

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "21");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(TEST_INPUT), "8");
    }
}
