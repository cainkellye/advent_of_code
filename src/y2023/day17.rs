use crate::utils::Grid;
use rayon::prelude::*;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/test17.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input17.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    let grid = parse_input(input_file);
    let treshold = Arc::new(Mutex::new(
        (1..grid.rows)
            .zip(1..grid.cols)
            .map(|(r, c)| (grid.item(r, c) + grid.item(r - 1, c).min(grid.item(r, c - 1))) as usize)
            .sum::<usize>(),
    ));
    usize::min(
        find_min_path(
            Pos(1, 0),
            &grid,
            vec![Pos(0, 0)],
            vec![Towards::Down],
            grid.item(1, 0) as usize,
            treshold.clone(),
        )
        .unwrap_or(999),
        find_min_path(
            Pos(0, 1),
            &grid,
            vec![Pos(0, 0)],
            vec![Towards::Right],
            grid.item(0, 1) as usize,
            treshold,
        )
        .unwrap_or(999),
    )
}

fn find_min_path(
    from: Pos,
    grid: &Grid,
    prev_moves: Vec<Pos>,
    prev_directions: Vec<Towards>,
    current_weight: usize,
    treshold: Arc<Mutex<usize>>,
) -> Option<usize> {
    if current_weight >= *treshold.lock().unwrap() {
        return None;
    }
    //println!("Prev: {}, {from:?}, {current_weight} < {treshold}", prev_moves.len());
    let bounds = Pos(grid.rows - 1, grid.cols - 1);
    let steps = get_valid_steps(from, &bounds, &prev_moves, &prev_directions);
    if steps.is_empty() {
        return None;
    }
    steps
        .into_par_iter()
        .filter_map(|(direction, pos)| {
            if pos == bounds {
                let weight = current_weight + grid.item(pos.0, pos.1) as usize;
                let mut treshold = treshold.lock().unwrap();
                if weight < *treshold {
                    println!(
                        "### Prev: {prev_moves:?} => {from:?} => {pos:?}, {weight} < {treshold}",
                    );
                    *treshold = weight;
                    Some(weight)
                } else {
                    None
                }
            } else {
                let prev_directions = if prev_directions.len() == 3 {
                    Vec::from_iter(prev_directions.iter().skip(1).cloned().chain([direction]))
                } else {
                    let mut clone = prev_directions.clone();
                    clone.push(direction);
                    clone
                };

                let mut prev_moves = prev_moves.clone();
                prev_moves.push(from);
                find_min_path(
                    pos,
                    &grid,
                    prev_moves,
                    prev_directions,
                    current_weight + grid.item(pos.0, pos.1) as usize,
                    treshold.clone(),
                )
            }
        })
        .min()
}

fn get_valid_steps(
    from: Pos,
    bounds: &Pos,
    prev_moves: &Vec<Pos>,
    prev_directions: &Vec<Towards>,
) -> Vec<(Towards, Pos)> {
    let mut valid = vec![];
    let last_direction = *prev_directions.last().unwrap();
    if prev_directions
        .iter()
        .rev()
        .skip(1)
        .any(|mv| mv != &last_direction)
    {
        if let Some(new_pos) = from.step(last_direction, &bounds) {
            if !prev_moves.contains(&new_pos) {
                valid.push((last_direction, new_pos));
            }
        }
    }
    for direction in [
        last_direction.clockwise(),
        last_direction.counter_clockwise(),
    ] {
        if let Some(new_pos) = from.step(direction, &bounds) {
            if !prev_moves.contains(&new_pos) {
                valid.push((direction, new_pos));
            }
        }
    }
    valid
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file);
    0
}

fn parse_input(input_file: &str) -> Grid {
    Grid::new(
        iter_lines_from(input_file)
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec()
            })
            .collect_vec(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Towards {
    Up,
    Down,
    Left,
    Right,
}
impl Towards {
    fn clockwise(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    fn counter_clockwise(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Pos(usize, usize);
impl Pos {
    #[rustfmt::skip]
    fn step(&self, towards: Towards, bounds: &Pos) -> Option<Pos> {
        match towards {
            Towards::Up => { if self.0 > 0 { Some(Pos(self.0 - 1, self.1)) } else { None } }
            Towards::Down => { if self.0 < bounds.0 { Some(Pos(self.0 + 1, self.1)) } else { None } }
            Towards::Left => { if self.1 > 0 { Some(Pos(self.0, self.1 - 1)) } else { None } }
            Towards::Right => { if self.1 < bounds.1 { Some(Pos(self.0, self.1 + 1)) } else { None } }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test17.txt"), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test17.txt"), 0);
    }
}
