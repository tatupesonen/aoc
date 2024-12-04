use crate::Solution;

const SUB: &str = "XMAS";

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let mtx: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut permutations: Vec<String> = vec![];

        let num_cols = mtx[0].len();
        let num_rows = mtx.len();
        // Rows
        for row in input.lines() {
            permutations.push(row.to_owned());
        }

        // Cols
        for col in 0..num_cols {
            let mut column = String::with_capacity(num_rows);
            for row in 0..num_rows {
                column.push(mtx[row][col]);
            }
            permutations.push(column.clone());
        }

        // Primary diagonals
        for start in 0..(num_rows + num_cols - 1) {
            let mut diag = String::new();
            for row in 0..num_rows {
                let col = start as isize - row as isize;
                if col >= 0 && col < num_cols as isize {
                    diag.push(mtx[row][col as usize]);
                }
            }
            permutations.push(diag.clone());
        }

        // Secondary diagonals
        for start in 0..(num_rows + num_cols - 1) {
            let mut diag = String::new();
            for row in 0..num_rows {
                let col = start as isize - row as isize;
                if col >= 0 && col < num_cols as isize {
                    diag.push(mtx[row][num_cols - 1 - col as usize]);
                }
            }
            permutations.push(diag.clone());
        }

        let mut count = 0;
        for line in permutations {
            count += line
                .as_bytes()
                .windows(SUB.len())
                .filter(|&w| w == SUB.as_bytes())
                .count();

            let row_rev = line.chars().rev().collect::<String>();
            count += row_rev
                .as_bytes()
                .windows(SUB.len())
                .filter(|&w| w == SUB.as_bytes())
                .count();
        }

        count.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        // thank god part 2 is WAY easier.
        let mtx: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let num_cols = mtx[0].len();
        let num_rows = mtx.len();
        let mut a_coords = vec![];

        for row in 1..(num_rows - 1) {
            for col in 1..(num_cols - 1) {
                if mtx[row][col] == 'A' {
                    a_coords.push((row, col))
                }
            }
        }
        // check original coords
        let mut count = 0;
        for a in a_coords {
            // check [-1, -1] up left
            // check [1, 1] down right
            let ul = mtx[a.0 - 1][a.1 - 1];
            let dr = mtx[a.0 + 1][a.1 + 1];

            // check [-1, 1] up right
            // check [1, -1] down left
            let ur = mtx[a.0 - 1][a.1 + 1];
            let dl = mtx[a.0 + 1][a.1 - 1];

            let urdl = format!("{}{}", ur, dl);
            let uldr = format!("{}{}", ul, dr);

            if (urdl == "SM" || urdl == "MS") && (uldr == "SM" || uldr == "MS") {
                count += 1;
            }
        }

        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../inputs/04/test-input.txt");
    const INPUT: &str = include_str!("../../inputs/04/input.txt");

    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(Problem.part_one(TEST_INPUT), "18");
    }

    #[test]
    fn part2_test() {
        assert_eq!(Problem.part_two(TEST_INPUT), "9");
    }

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(INPUT), "2547");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(INPUT), "1939");
    }
}
