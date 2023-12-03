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
    [day01_1, day01_2],
    [day02_1, day02_2],
    [day03_1, day03_2],
    [day04_1, day04_2],
    [day05_1, day05_2],
];

fn day05_1() {}

fn day05_2() {}

fn day04_1() {}

fn day04_2() {}

fn day03_1() {
    let (symbols, numbers) = day03_parse_input("res/2023/input03.txt");
    let sum: usize = numbers
        .iter()
        .filter(|(_, nloc)| symbols.iter().any(|(_, sloc)| nloc.is_near(sloc)))
        .map(|&(n, _)| n)
        .sum();
    println!("{sum}");
}

fn day03_2() {
    let (symbols, numbers) = day03_parse_input("res/2023/input03.txt");
    let gears = symbols.into_iter().filter(|(s, _)| *s == '*');
    let sum: usize = gears
        .map(|(_, gloc)| {
            let near_numbers = numbers.iter().filter(|(_, nloc)| nloc.is_near(&gloc)).collect_vec();
            if near_numbers.len() == 2 {
                near_numbers[0].0 * near_numbers[1].0
            } else { 0 }
        })
        .sum();
    println!("{sum}");
}

#[derive(Debug)]
/// Line, col start, col end
struct Location(usize, usize, usize);
impl Location {
    fn is_near(&self, other: &Location) -> bool {
        if self.0.abs_diff(other.0) > 1 {
            //lines are too far
            return false;
        }
        self.2 + 1 >= other.1 && other.2 + 1 >= self.1
    }
}

fn day03_parse_input(input_path: &str) -> (Vec<(char, Location)>, Vec<(usize, Location)>) {
    let mut symbols: Vec<(char, Location)> = vec![];
    let mut numbers: Vec<(usize, Location)> = vec![];
    let mut num_start_idx = 0;
    let mut num = String::new();
    for (line_idx, line) in iter_lines_from(input_path).enumerate() {
        if !num.is_empty() {
            numbers.push((
                num.parse().unwrap(),
                Location(line_idx - 1, num_start_idx, num_start_idx + num.len() - 1),
            ));
            num.clear();
        }
        for (char_idx, ch) in line.chars().enumerate() {
            match ch {
                '0'..='9' => {
                    let num: &mut String = &mut num;
                    let start: &mut usize = &mut num_start_idx;
                    if num.is_empty() {
                        *start = char_idx;
                    }
                    num.push(ch);
                }
                ch => {
                    {
                        if !num.is_empty() {
                            numbers.push((
                                num.parse().unwrap(),
                                Location(line_idx, num_start_idx, char_idx - 1),
                            ));
                            num.clear();
                        }
                    };
                    match ch {
                        '.' => continue,
                        _ => symbols.push((ch, Location(line_idx, char_idx, char_idx))),
                    }
                }
            }
        }
    }
    (symbols, numbers)
}

fn day02_1() {
    let limit = [12, 13, 14];
    let sum: u32 = iter_lines_from("res/2023/input02.txt")
        .map(|l| {
            //Game 5: 1 red, 3 blue, 15 green; 13 green, 2 blue; 6 green; 6 green, 8 blue; 4 green, 9 blue, 1 red
            l.split_once(": ")
                .expect("input error")
                .1
                .split("; ")
                .map(|set_line| {
                    set_line.split(", ").fold([0, 0, 0], |[r, g, b], part| {
                        match part.split_once(' ') {
                            Some((num, "red")) => [num.parse::<usize>().unwrap(), g, b],
                            Some((num, "green")) => [r, num.parse::<usize>().unwrap(), b],
                            Some((num, "blue")) => [r, g, num.parse::<usize>().unwrap()],
                            _ => unreachable!("Part"),
                        }
                    })
                })
                .collect_vec()
        })
        .enumerate()
        .filter(|(_, game)| {
            game.iter()
                .all(|set| set.iter().zip(limit.iter()).all(|(s, l)| s <= l))
        })
        .map(|(idx, _)| idx as u32 + 1)
        .sum();
    println!("{:?}", sum);
}

fn day02_2() {
    let sum: usize = iter_lines_from("res/2023/input02.txt")
        .map(|l| {
            //Game 5: 1 red, 3 blue, 15 green; 13 green, 2 blue; 6 green; 6 green, 8 blue; 4 green, 9 blue, 1 red
            l.split_once(": ")
                .expect("input error")
                .1
                .split("; ")
                .map(|set_line| {
                    set_line.split(", ").fold([0, 0, 0], |[r, g, b], part| {
                        match part.split_once(' ') {
                            Some((num, "red")) => [num.parse::<usize>().unwrap(), g, b],
                            Some((num, "green")) => [r, num.parse::<usize>().unwrap(), b],
                            Some((num, "blue")) => [r, g, num.parse::<usize>().unwrap()],
                            _ => unreachable!("Part"),
                        }
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
