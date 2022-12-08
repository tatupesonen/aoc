use crate::{
    utils::{self, transpose},
    Solution,
};

fn iter_row(vec: &Vec<Vec<i8>>, counts: &mut Vec<Vec<i8>>) {
    for (row_num, row) in vec.into_iter().enumerate() {
        let mut prev = -1;
        for (col_num, tree) in row.into_iter().enumerate() {
            match *tree > prev {
                true => {
                    prev = *tree;
                    counts[row_num][col_num] += 1;
                }
                _ => {}
            }
        }
    }
}

fn iter_cols(vec: &Vec<Vec<i8>>, counts: &mut Vec<Vec<i8>>) {
    for (y, row) in vec.iter().enumerate() {
        let mut prev = -1;
        for (x, col) in row.iter().enumerate() {
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

fn iter_rows_rev(vec: &Vec<Vec<i8>>, counts: &mut Vec<Vec<i8>>) {
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
}

fn iter_cols_rev(vec: &Vec<Vec<i8>>, counts: &mut Vec<Vec<i8>>) {
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
        iter_row(&map, &mut counts);
        iter_cols(&map, &mut counts);
        iter_rows_rev(&map, &mut counts);
        iter_cols_rev(&map, &mut counts);
        let count: u32 = counts
            .into_iter()
            .flatten()
            .filter_map(|e| if e > 0 { Some(1) } else { None })
            .sum();

        count.to_string()
    }

    fn part_two(&self, input: &str) -> String {
			todo!();
    }
}
// 01234
// |||||
// 30373 - 0
// 25512 - 1
// 65332 - 2
// 33549 - 3
// 35390 - 4

fn print_vec(v: &Vec<Vec<i8>>) {
    println!("***********");
    for row in v {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/8/test-input.txt");

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "21");
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(Problem.part_two(TEST_INPUT), "8");
    }
}
