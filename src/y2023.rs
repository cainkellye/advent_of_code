#![allow(unused_imports)]
use super::iter_lines_from;
use itertools::Itertools;

/// https://adventofcode.com/2023
pub fn solve(day: usize, part: usize) {
    println!("Solution for day {day} part {part} (2023)");
    SOLUTIONS[day - 1][part - 1]();
}

pub fn solve_all() {
    SOLUTIONS
        .iter()
        .enumerate()
        .for_each(|(day, [part1, part2])| {
            let day = day + 1;
            println!("Solution for day {day} part 1 (2023)");
            part1();
            println!("Solution for day {day} part 2 (2023)");
            part2();
        });
}

const SOLUTIONS: [[fn(); 2]; 5] = [
    [day01::part1, day01::part2],
    [day02::part1, day02::part2],
    [day03::part1, day03::part2],
    [day04::part1, day04::part2],
    [day05::part1, day05::part2],
];

mod day05;
mod day04;
mod day03;
mod day02;
mod day01;
