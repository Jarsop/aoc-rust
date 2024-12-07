use crate::utils::solver;

pub struct Solver;

impl solver::Solver<2024, 1> for Solver {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let mut cols: (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(|line| {
                let mut nums = line
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .take(2);
                (nums.next().unwrap(), nums.next().unwrap())
            })
            .unzip();

        cols.0.sort();
        cols.1.sort();

        cols.0
            .iter()
            .zip(cols.1.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        42
    }
}
