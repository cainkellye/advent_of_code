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
            return false; //lines are too far
        }
        self.2 + 1 >= other.1 && other.2 + 1 >= self.1
    }
}

type SymbolOccurance = (char, Location);
type NumberOccurance = (usize, Location);

fn parse_input(input_path: &str) -> (Vec<SymbolOccurance>, Vec<NumberOccurance>) {
    let mut symbols: Vec<(char, Location)> = vec![];
    let mut numbers: Vec<(usize, Location)> = vec![];
    let mut num_start_idx = 0;
    let mut num = String::new();
    for (line_idx, line) in iter_lines_from(input_path).enumerate() {
        push_number(&mut num, &mut numbers, line_idx.max(1) - 1, num_start_idx);
        for (char_idx, ch) in line.chars().enumerate() {
            if matches!(ch, '0'..='9') {
                let num: &mut String = &mut num;
                let start: &mut usize = &mut num_start_idx;
                if num.is_empty() {
                    *start = char_idx;
                }
                num.push(ch);
            } else {
                push_number(&mut num, &mut numbers, line_idx, num_start_idx);
                if ch != '.' {
                    symbols.push((ch, Location(line_idx, char_idx, char_idx)))
                }
            }
        }
    }
    (symbols, numbers)
}

fn push_number(
    num: &mut String,
    numbers: &mut Vec<NumberOccurance>,
    line_idx: usize,
    num_start_idx: usize,
) {
    if !num.is_empty() {
        numbers.push((
            num.parse().unwrap(),
            Location(line_idx, num_start_idx, num_start_idx + num.len() - 1),
        ));
        num.clear();
    }
}
