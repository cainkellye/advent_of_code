#![allow(unused)]
use super::*;
use ansi_hex_color::colored;

pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input18.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input18.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    for input in parse_input(input_file) {
        let text = colored(&input.2, "#000000", &format!("{input:?}"));
        println!("{text}");
    }
    0
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file);
    0
}

fn parse_input(input_file: &str) -> impl Iterator<Item = (u8, u8, String)> {
    iter_lines_from(input_file).map(|line| {
        let (direction, rest) = line.split_once(' ').unwrap();
        let (count, color) = rest.split_once(' ').unwrap();
        let color = color[1..color.len() - 1].to_owned();
        (direction.as_bytes()[0], count.parse().unwrap(), color)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test18.txt"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test18.txt"), 0);
    }
}
