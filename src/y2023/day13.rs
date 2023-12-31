use super::*;
use crate::utils::Grid;

pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input13.txt")); // 33975
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input13.txt")); // 29083
}

fn part1_internal(input_file: &str) -> usize {
    parse_input(input_file)
        .into_iter()
        .map(|grid| part1::find_mirror(&grid))
        .sum()
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file)
        .into_iter()
        .map(|grid| part2::find_mirror(&grid))
        .sum()
}

mod part1;
mod part2;

fn parse_input(input_file: &str) -> Vec<Grid> {
    iter_lines_from(input_file)
        .batching(|lines| {
            let grid = lines
                .take_while(|line| !line.is_empty())
                .map(|line| line.as_bytes().to_vec())
                .collect_vec();
            if grid.is_empty() {
                None
            } else {
                Some(Grid::new(grid))
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod test;
