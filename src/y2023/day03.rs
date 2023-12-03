use super::*;
pub fn part1() {
    let (symbols, numbers) = parse_input("res/2023/input03.txt");
    let sum: usize = numbers
        .iter()
        .filter(|(_, nloc)| symbols.iter().any(|(_, sloc)| nloc.is_near(sloc)))
        .map(|&(n, _)| n)
        .sum();
    println!("{sum}");
}

pub fn part2() {
    let (symbols, numbers) = parse_input("res/2023/input03.txt");
    let gears = symbols.into_iter().filter(|(s, _)| *s == '*');
    let sum: usize = gears
        .map(|(_, gloc)| {
            let near_numbers = numbers
                .iter()
                .filter(|(_, nloc)| nloc.is_near(&gloc))
                .collect_vec();
            if near_numbers.len() == 2 {
                near_numbers[0].0 * near_numbers[1].0
            } else {
                0
            }
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

fn parse_input(input_path: &str) -> (Vec<(char, Location)>, Vec<(usize, Location)>) {
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
