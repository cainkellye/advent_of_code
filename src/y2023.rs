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

const SOLUTIONS: [[fn(); 2]; 4] = [
    [day01_1, day01_2],
    [day02_1, day02_2],
    [day03_1, day03_2],
    [day04_1, day04_2],
];

fn day04_1() {}

fn day04_2() {}

fn day03_1() {}

fn day03_2() {}

fn day02_1() {
    let limit = (12, 13, 14);
    let sum: u32 = iter_lines_from("res/2023/input02.txt")
        .map(|l| {
            //Game 5: 1 red, 3 blue, 15 green; 13 green, 2 blue; 6 green; 6 green, 8 blue; 4 green, 9 blue, 1 red
            l.split_once(':')
                .expect("input error")
                .1
                .split(';')
                .map(|set_line| {
                    set_line.split(',').fold((0, 0, 0), |mut set, part| {
                        match part.trim().split_once(' ') {
                            Some((num, "red")) => set.0 += num.parse::<u32>().unwrap(),
                            Some((num, "green")) => set.1 += num.parse::<u32>().unwrap(),
                            Some((num, "blue")) => set.2 += num.parse::<u32>().unwrap(),
                            _ => unreachable!("Part"),
                        }
                        set
                    })
                })
                .collect_vec()
        })
        .enumerate()
        .filter(|(_, game)| {
            game.iter()
                .all(|set| set.0 <= limit.0 && set.1 <= limit.1 && set.2 <= limit.2)
        })
        .map(|(idx, _)| idx as u32 + 1)
        .sum();
    println!("{:?}", sum);
}

fn day02_2() {
    let sum: usize = iter_lines_from("res/2023/input02.txt")
        .map(|l| {
            //Game 5: 1 red, 3 blue, 15 green; 13 green, 2 blue; 6 green; 6 green, 8 blue; 4 green, 9 blue, 1 red
            l.split_once(':')
                .expect("input error")
                .1
                .split(';')
                .map(|set_line| {
                    set_line.split(',').fold([0, 0, 0], |mut set, part| {
                        match part.trim().split_once(' ') {
                            Some((num, "red")) => set[0] += num.parse::<usize>().unwrap(),
                            Some((num, "green")) => set[1] += num.parse::<usize>().unwrap(),
                            Some((num, "blue")) => set[2] += num.parse::<usize>().unwrap(),
                            _ => unreachable!("Part"),
                        }
                        set
                    })
                })
                .collect_vec()
        })
        .map(|game| {
            game.iter()
                .fold([0, 0, 0], |set, part| {
                    [
                        set[0].max(part[0]),
                        set[1].max(part[1]),
                        set[2].max(part[2]),
                    ]
                })
                .iter()
                .product::<usize>()
        })
        .sum();
    println!("{:?}", sum);
}

fn day01_1() {
    let sum: u32 = iter_lines_from("res/2023/input01.txt")
        .map(|l| {
            let digits = l.chars().filter_map(|c| c.to_digit(10)).collect_vec();
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum();
    println!("{:?}", sum);
}

fn day01_2() {
    let digits: Vec<(String, _)> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .flat_map(|(i, &digit)| [(digit.into(), i + 1), (format!("{}", i + 1), i + 1)])
    .collect();

    let sum: usize = iter_lines_from("res/2023/input01.txt")
        .map(|l| {
            let (_, first) = digits
                .iter()
                .min_by_key(|(s, _)| l.find(s.as_str()).unwrap_or(usize::MAX))
                .unwrap();
            let (_, last) = digits
                .iter()
                .max_by_key(|(s, _)| l.rfind(s.as_str()).map_or(-1, |x| x as i32))
                .unwrap();
            first * 10 + last
        })
        .sum();
    println!("{:?}", sum);
}
