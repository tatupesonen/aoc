use std::{collections::HashMap, str::FromStr};

use crate::Solution;
const TOTAL_AVAIL_DISK_SPACE: usize = 70_000_000;
const NEEDED_TO_UPDATE: usize = 30_000_000;

#[derive(Debug)]
struct File {
    filesize: usize,
}
#[derive(Debug)]
enum Token {
    Command(CommandType),
    File(File),
    Directory(String),
}

#[derive(Debug)]
enum CommandType {
    List,
    GoRoot(),
    GoDown(String),
    GoUp(),
}

impl FromStr for Token {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.starts_with('$') {
            let mut it = line.split_whitespace().skip(1);
            match it.next().unwrap() {
                "cd" => match it.next().unwrap() {
                    "/" => Ok(Token::Command(CommandType::GoRoot())),
                    ".." => Ok(Token::Command(CommandType::GoUp())),
                    a => Ok(Token::Command(CommandType::GoDown(a.into()))),
                },
                _ => Ok(Token::Command(CommandType::List)),
            }
        } else {
            let mut it = line.split_whitespace();
            match it.next().unwrap() {
                "dir" => Ok(Token::Directory(it.next().unwrap().into())),
                filesize => Ok(Token::File(File {
                    filesize: filesize.parse().unwrap(),
                })),
            }
        }
    }
}

fn get_dir_sizes(input: Vec<Token>) -> HashMap<String, usize> {
    let mut stack: Vec<String> = vec![];
    let mut dirs: HashMap<String, Vec<File>> = HashMap::new();

    // Go through each token and build the tree
    for token in input {
        match token {
            Token::Command(command) => match command {
                CommandType::GoRoot() => {
                    // println!("going to root");
                    stack.push("".into());
                }
                CommandType::GoDown(target) => {
                    // println!("descending into {}", target);
                    stack.push(target);
                }
                CommandType::GoUp() => {
                    stack.pop();
                }
                _ => {}
            },
            Token::File(file) => {
                let path = stack.join("/");
                let target = dirs.entry(path).or_insert(vec![]);
                target.push(file);
            }
            _ => {}
        }
    }

    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    for (path, files_in_dir) in dirs.iter() {
        let dir_paths: Vec<&str> = path.split('/').collect();
        let size: usize = files_in_dir.iter().map(|e| e.filesize).sum();

        for i in 0..dir_paths.len() {
            dir_sizes
                .entry(
                    dir_paths[0..=i]
                        .iter()
                        .cloned()
                        .intersperse("/")
                        .collect::<String>(),
                )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }
    dir_sizes
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let tokens = input
            .lines()
            .map(|e| e.parse().unwrap())
            .collect::<Vec<Token>>();

        let dir_sizes = get_dir_sizes(tokens);

        dir_sizes
            .values()
            .filter(|&&s| s < 100_000)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let tokens = input
            .lines()
            .map(|e| e.parse().unwrap())
            .collect::<Vec<Token>>();

        let dir_sizes = get_dir_sizes(tokens);
        let space_taken = dir_sizes.get("").unwrap();

        let cur_free_space = TOTAL_AVAIL_DISK_SPACE - space_taken;
        let need_to_free_min = NEEDED_TO_UPDATE - cur_free_space;

        let mut to_consider = dir_sizes
            .iter()
            .filter(|(_, &s)| s >= need_to_free_min)
            .map(|(_, s)| s)
            .collect::<Vec<&usize>>();

        to_consider.sort();
        to_consider.first().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/7/test-input.txt");

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "95437");
    }

    #[test]
    fn part2() {
        assert_eq!(Problem.part_two(TEST_INPUT), "24933642");
    }
}
