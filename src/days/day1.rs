use crate::Problem;

pub struct Solution;
impl Problem for Solution {
    fn part_one(input: &str) -> () {
        let output = input
            .split("\n\n")
            .map(|e| {
                e.split("\n")
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .max()
            .unwrap();
        println!("{:?}", output);
    }

    fn part_two(input: &str) -> () {
        let mut vec: Vec<u32> = input
            .split("\n\n")
            .map(|e| {
                e.split("\n")
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .collect();
        vec.sort_by(|a, b| b.cmp(a));
        let output: u32 = vec.into_iter().take(3).sum();
        println!("{:?}", output);
    }
}
