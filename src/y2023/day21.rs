#![allow(unused)]
use std::collections::{HashSet, VecDeque};
use num::Integer;

use super::*;
use crate::utils::Grid;

pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input21.txt", 64, false)); // 3338 < ? < 3597
    //println!("{:?}", part1_internal("res/2023/test21.txt", 6, true));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input21.txt"));
}

fn part1_internal(input_file: &str, needed_steps: usize, visualize: bool) -> usize {
    let grid = parse_input(input_file);
    let start = grid
        .buffer
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .position(|&c| c == b'S')
                .map(|col| Pos(row, col))
        })
        .unwrap();
    let distances: Vec<Vec<Option<usize>>> = grid
        .buffer
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, &c)| {
                    if c == b'#' {
                        None
                    } else {
                        Some(distance(Pos(row, col), start, &grid))
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    if visualize {
        for row in 0..grid.rows {
            for col in 0..grid.cols {
                if let Some(dist) = distances[row][col] {
                    print!("{}{dist:<3}", grid.item(row, col) as char);
                } else {
                    print!("{}   ", grid.item(row, col) as char)
                }
            }
            println!();
        }
    }
    distances
        .into_iter()
        .flat_map(|line| line.into_iter())
        .filter_map(|x| {
            x.and_then(|c| {
                if c <= needed_steps && (needed_steps - c) % 2 == 0 {
                    Some(())
                } else {
                    None
                }
            })
        })
        .count()
}

fn distance(from: Pos, to: Pos, grid: &Grid) -> usize {
    let deltas = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut steps = VecDeque::<(Pos, usize)>::new();
    let mut touched = HashSet::<Pos>::new();
    for pos in deltas
        .iter()
        .filter_map(|&delta| from.with_delta(delta, grid.rows, grid.cols))
        .filter(|&Pos(row, col)| grid.item(row, col) != b'#')
    {
        steps.push_back((pos, 1));
    }
    while let Some((pos, dist)) = steps.pop_front() {
        if pos == to {
            return dist;
        }
        for pos in deltas
            .iter()
            .filter_map(|&delta| pos.with_delta(delta, grid.rows, grid.cols))
            .filter(|&Pos(row, col)| grid.item(row, col) != b'#')
            .sorted_by_key(|pos| pos.0.abs_diff(to.0) + pos.1.abs_diff(to.1))
        {
            if !touched.contains(&pos) {
                steps.push_back((pos, dist + 1));
                touched.insert(pos);
            }
        }
    }
    999999
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file);
    0
}

fn parse_input(input_file: &str) -> Grid {
    Grid::new(
        iter_lines_from(input_file)
            .map(|line| line.as_bytes().to_vec())
            .collect_vec(),
    )
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Pos(usize, usize);
impl Pos {
    fn with_delta(&self, (d_row, d_col): (isize, isize), rows: usize, cols: usize) -> Option<Pos> {
        if d_row < 0 && self.0 == 0 || d_col < 0 && self.1 == 0 {
            return None;
        }
        if d_row > 0 && self.0 == rows - 1 || d_col > 0 && self.1 == cols - 1 {
            return None;
        }
        let new_row = (self.0 as isize + d_row) as usize;
        let new_col = (self.1 as isize + d_col) as usize;
        Some(Pos(new_row, new_col))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test21.txt", 6, false), 16);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test21.txt"), 0);
    }
}
