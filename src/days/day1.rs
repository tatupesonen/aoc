use crate::Solution;

pub struct Problem;
impl Solution for Problem {
    fn part_one(input: &str) -> String {
        let output = input
            .split("\n\n")
            .map(|e| {
                e.split('\n')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .max()
            .unwrap();
        println!("{:?}", output);
        output.to_string()
    }

    fn part_two(input: &str) -> String {
        let mut vec: Vec<u32> = input
            .split("\n\n")
            .map(|e| {
                e.split('\n')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .collect();
        vec.sort_by(|a, b| b.cmp(a));
        let output: u32 = vec.into_iter().take(3).sum();
        println!("{:?}", output);
        output.to_string()
    }
}
