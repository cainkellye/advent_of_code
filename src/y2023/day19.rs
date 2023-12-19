#![allow(unused)]
use std::collections::HashMap;

use super::*;

pub fn part1() {
    println!("{:?}", part1::part1_internal("res/2023/input19.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input19.txt"));
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file);
    0
}

fn parse_input(input_file: &str) -> () {
    iter_lines_from(input_file).map(|line| line);
}

mod part1;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test19.txt"), 0);
    }
}
