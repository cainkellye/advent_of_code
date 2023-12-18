#![allow(unused)]
use std::collections::HashMap;

use super::*;
use ansi_hex_color::colored;

pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input18.txt", true)); // 47675
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input18.txt"));
}

fn part1_internal(input_file: &str, visualize: bool) -> usize {
    let mut digged = HashMap::new();
    let mut position = Pos(0, 0);
    digged.insert(position, "#909090".to_owned());
    for (direction, count, color_hex) in parse_input(input_file) {
        for _ in 0..(count as isize) {
            match direction {
                b'U' => position.0 -= 1,
                b'D' => position.0 += 1,
                b'L' => position.1 -= 1,
                b'R' => position.1 += 1,
                _ => unreachable!(),
            }
            digged.insert(position, color_hex.clone());
        }
    }
    calculate_volume(digged, visualize)
}

fn part2_internal(input_file: &str) -> usize {
    let mut digged = HashMap::new();
    let mut position = Pos(0, 0);
    digged.insert(position, "#909090".to_owned());
    for (_, _, color_hex) in parse_input(input_file) {
        // last hex char: 0 means R, 1 means D, 2 means L, and 3 means U.
        let count = usize::from_str_radix(&color_hex[1..6], 16).unwrap();
        let direction = match &color_hex[6..7] {
            "0" => b'R',
            "1" => b'D',
            "2" => b'L',
            "3" => b'U',
            _ => unreachable!(),
        };
        for _ in 0..(count as isize) {
            match direction {
                b'U' => position.0 -= 1,
                b'D' => position.0 += 1,
                b'L' => position.1 -= 1,
                b'R' => position.1 += 1,
                _ => unreachable!(),
            }
            digged.insert(position, color_hex.clone());
        }
    }
    calculate_volume(digged, false)
}

fn calculate_volume(digged: HashMap<Pos, String>, visualize: bool) -> usize {
    let min_row = *digged.keys().map(|Pos(row, _)| row).min().unwrap();
    let min_col = *digged.keys().map(|Pos(_, col)| col).min().unwrap();
    let max_row = *digged.keys().map(|Pos(row, _)| row).max().unwrap();
    let max_col = *digged.keys().map(|Pos(_, col)| col).max().unwrap();
    let mut cubic_meters = 0;
    let mut inside = false;
    let mut corner = None;
    // true if there was a wall above, false if below
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if let Some(color_hex) = digged.get(&Pos(row, col)) {
                if visualize {
                    print!("{}", colored(color_hex, "#000000", "#"));
                }
                if digged.get(&Pos(row, col + 1)).is_none() {
                    if corner.is_some_and(|above_wall| {
                        digged.contains_key(&Pos(if above_wall { row + 1 } else { row - 1 }, col))
                    }) || corner.is_none()
                    {
                        inside ^= true;
                        corner = None;
                    }
                } else {
                    let above_wall = digged.contains_key(&Pos(row - 1, col));
                    let below_wall = digged.contains_key(&Pos(row + 1, col));
                    if above_wall || below_wall {
                        corner = Some(above_wall);
                    }
                }
                cubic_meters += 1;
            } else if inside {
                if visualize {
                    print!("{}", colored("#909090", "#000000", "X"));
                }
                cubic_meters += 1;
            } else {
                corner = None;
                if visualize {
                    print!(" ");
                }
            }
        }
        corner = None;
        if visualize {
            println!();
        }
    }
    cubic_meters
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Pos(isize, isize);

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
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test18.txt"), 0);
    }
}
