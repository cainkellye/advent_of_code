#![allow(unused_imports)]
use super::iter_lines_from;
use crate::utils::time;
use itertools::Itertools;

/// https://adventofcode.com/2023
pub fn solve(day: usize, part: usize) {
    println!("Solution for day {day} part {part} (2023)");
    time(&SOLUTIONS[day - 1][part - 1]);
}

pub fn solve_all() {
    SOLUTIONS
        .iter()
        .enumerate()
        .for_each(|(day, [part1, part2])| {
            let day = day + 1;
            println!("Solution for day {day} part 1 (2023)");
            time(part1);
            println!("Solution for day {day} part 2 (2023)");
            time(part2);
        });
}

const SOLUTIONS: &[[fn(); 2]] = &[
    [day01::part1, day01::part2],
    [day02::part1, day02::part2],
    [day03::part1, day03::part2],
    [day04::part1, day04::part2],
    [day05::part1, day05::part2],
    [day06::part1, day06::part2],
    [day07::part1, day07::part2],
    [day08::part1, day08::part2],
    [day09::part1, day09::part2],
    [day10::part1, day10::part2],
    [day11::part1, day11::part2],
    [day12::part1, day12::part2],
    [day13::part1, day13::part2],
    [day14::part1, day14::part2],
    [day15::part1, day15::part2],
    [day16::part1, day16::part2],
    [day17::part1, day17::part2],
    [day18::part1, day18::part2],
    [day19::part1, day19::part2],
];

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
