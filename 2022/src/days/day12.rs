use owo_colors::OwoColorize;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    time::Duration,
};

use crate::Solution;

fn print_map(vec: &Vec<Vec<usize>>, unvis: &HashSet<Point>) {
    let mut output = String::new();
    for (y, line) in vec.into_iter().enumerate() {
        for (x, val) in line.into_iter().enumerate() {
            if unvis.contains(&(x, y)) {
                output += &std::char::from_u32(*val as u32 + 'a' as u32)
                    .unwrap()
                    .dimmed()
                    .to_string()
            } else {
                output += &std::char::from_u32(*val as u32 + 'a' as u32)
                    .unwrap()
                    .bright_green()
                    .to_string()
            }
        }
        output += "\n";
    }
    print!("{}", output);
    std::thread::sleep(Duration::from_millis(10));
}

fn dijkstra(map: HashMap<Point, HashSet<Point>>, start: Point) -> HashMap<Point, usize> {
    // max out the distance for every node
    let mut dist: HashMap<Point, usize> =
        HashMap::from_iter(map.keys().map(|&key| (key, usize::MAX - 1)));
    dist.insert(start, 0);

    let mut unvisited: HashSet<Point> = HashSet::from_iter(map.keys().map(|&k| k));

    while !unvisited.is_empty() {
        let cur = *unvisited.iter().min_by_key(|&k| dist[k]).unwrap();

        unvisited.remove(&cur);

        for &neighbor in map[&cur].iter().filter(|node| unvisited.contains(node)) {
            let potential_dist = dist[&cur] + 1;
            if potential_dist < dist[&neighbor] {
                dist.insert(neighbor, potential_dist);
            };
        }
    }
    dist
}

type Point = (usize, usize);
fn create_graph(map: Vec<Vec<usize>>) -> HashMap<Point, HashSet<Point>> {
    let mut graph: HashMap<Point, HashSet<Point>> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &item) in row.iter().enumerate() {
            let mut neighbors: HashSet<Point> = HashSet::new();
            // Perform checks for each neighbor and determine if we can step up there
            if x > 0 {
                // If it's less than or equal to 1, we can step there
                // check below
                if map[y][x - 1] as isize - item as isize <= 1 {
                    neighbors.insert((x - 1, y));
                }
            }
            if x < row.len() - 1 {
                if map[y][x + 1] as isize - item as isize <= 1 {
                    neighbors.insert((x + 1, y));
                }
            }
            if y > 0 {
                if map[y - 1][x] as isize - item as isize <= 1 {
                    neighbors.insert((x, y - 1));
                }
            }
            if y < map.len() - 1 {
                if map[y + 1][x] as isize - item as isize <= 1 {
                    neighbors.insert((x, y + 1));
                }
            }
            graph.insert((x, y), neighbors);
        }
    }
    graph
}

pub struct Problem;
impl Solution for Problem {
    fn part_one(&self, input: &str) -> String {
        let mut start: Option<Point> = None;
        let mut end: Option<Point> = None;

        let map: Vec<Vec<usize>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = Some((x as usize, y as usize));
                            0
                        }
                        'E' => {
                            end = Some((x as usize, y as usize));
                            25
                        }
                        _ => c as usize - 'a' as usize,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        // print_map(map);
        let graph = create_graph(map.clone());
        let dist = dijkstra(graph, start.unwrap());
        let result = dist[&end.unwrap()];

        result.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut start: Option<Point> = None;
        let mut end = Vec::new();

        let map: Vec<Vec<usize>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            end.push(Some((x as usize, y as usize)));
                            0
                        }
                        'E' => {
                            start = Some((x as usize, y as usize));
                            25
                        }
                        _ => c as usize - 'a' as usize,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let graph = create_graph(map.clone());
        let dist = dijkstra(graph, start.unwrap());
        let result = end
            .iter()
            .filter_map(|&e| e)
            .map(|e| dist[&e])
            .min()
            .unwrap();

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/12/test-input.txt");

    #[test]
    fn part1() {
        assert_eq!(Problem.part_one(TEST_INPUT), "31");
    }

    #[test]
		#[ignore]
    fn part2() {
        assert_eq!(&Problem.part_two(TEST_INPUT), "29");
    }
}
