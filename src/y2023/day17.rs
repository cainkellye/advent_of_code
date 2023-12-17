use crate::utils::{Grid, Part};
use num::traits::bounds;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input17.txt")); // 956
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input17.txt")); // 1106
}

type Minimums = Rc<RefCell<HashMap<(Pos, (Towards, u8)), usize>>>;

fn part1_internal(input_file: &str) -> usize {
    let grid = parse_input(input_file);
    let treshold = Rc::new(Cell::new(
        (1..grid.rows)
            .zip(1..grid.cols)
            .map(|(r, c)| (grid.item(r, c) + grid.item(r - 1, c).min(grid.item(r, c - 1))) as usize)
            .sum::<usize>(),
    ));
    let minimums: Minimums = Rc::new(RefCell::new(HashMap::new()));
    usize::min(
        find_min_path(
            Pos(1, 0),
            &grid,
            vec![Pos(0, 0)],
            (Towards::Down, 1),
            grid.item(1, 0) as usize,
            treshold.clone(),
            minimums.clone(),
            Part::One,
        )
        .unwrap_or(9999),
        find_min_path(
            Pos(0, 1),
            &grid,
            vec![Pos(0, 0)],
            (Towards::Right, 1),
            grid.item(0, 1) as usize,
            treshold,
            minimums.clone(),
            Part::One,
        )
        .unwrap_or(9999),
    )
}

fn find_min_path(
    from: Pos,
    grid: &Grid,
    prev_moves: Vec<Pos>,
    prev_directions: (Towards, u8),
    current_weight: usize,
    treshold: Rc<Cell<usize>>,
    minimums: Minimums,
    part: Part,
) -> Option<usize> {
    if current_weight >= treshold.get() {
        return None;
    }
    let bounds = Pos(grid.rows - 1, grid.cols - 1);
    let mut steps = get_valid_steps(from, bounds, prev_directions, part);
    steps.sort_by_key(|(_, Pos(row, col))| bounds.0 - row + bounds.1 - col);
    steps
        .into_iter()
        .filter_map(|(direction, pos)| {
            let weight = current_weight + grid.item(pos.0, pos.1) as usize;
            let (last_direction, dir_count) = prev_directions;
            let prev_directions = if last_direction == direction {
                (direction, dir_count + 1)
            } else {
                (direction, 1)
            };
            if pos == bounds {
                if part == Part::One || prev_directions.1 >= 4 {
                    if weight < treshold.get() {
                        println!(
                            //"### Prev: {prev_moves:?} => {from:?} => {pos:?}, {weight} < {treshold}",
                            "### New {weight} < {treshold:?}",
                        );
                        treshold.set(weight);
                        Some(weight)
                    } else {
                        //println!("### Threshold not beaten: {treshold:?}");
                        None
                    }
                } else {
                    None
                }
            } else {
                let min = minimums.borrow().get(&(pos, prev_directions)).copied();
                if min.is_some_and(|min| min <= weight) {
                    return None;
                } else {
                    minimums.borrow_mut().insert((pos, prev_directions), weight);
                }

                let mut prev_moves = prev_moves.clone();
                prev_moves.push(from);
                find_min_path(
                    pos,
                    grid,
                    prev_moves,
                    prev_directions,
                    weight,
                    treshold.clone(),
                    minimums.clone(),
                    part,
                )
            }
        })
        .min()
}

fn get_valid_steps(
    from: Pos,
    bounds: Pos,
    prev_directions: (Towards, u8),
    part: Part,
) -> Vec<(Towards, Pos)> {
    let mut valid = vec![];
    let (last_direction, dir_count) = prev_directions;
    if part == Part::One {
        if dir_count < 3 {
            if let Some(new_pos) = from.step(last_direction, &bounds) {
                valid.push((last_direction, new_pos));
            }
        }
        for direction in [
            last_direction.clockwise(),
            last_direction.counter_clockwise(),
        ] {
            if let Some(new_pos) = from.step(direction, &bounds) {
                valid.push((direction, new_pos));
            }
        }
    } else {
        if dir_count < 10 {
            if let Some(new_pos) = from.step(last_direction, &bounds) {
                valid.push((last_direction, new_pos));
            }
        }
        if dir_count >= 4 {
            for direction in [
                last_direction.clockwise(),
                last_direction.counter_clockwise(),
            ] {
                if let Some(new_pos) = from.step(direction, &bounds) {
                    valid.push((direction, new_pos));
                }
            }
        }
    }
    valid
}

fn part2_internal(input_file: &str) -> usize {
    let grid = parse_input(input_file);
    let treshold = Rc::new(Cell::new(1400));
    let minimums: Minimums = Rc::new(RefCell::new(HashMap::new()));
    usize::min(
        find_min_path(
            Pos(1, 0),
            &grid,
            vec![Pos(0, 0)],
            (Towards::Down, 1),
            grid.item(1, 0) as usize,
            treshold.clone(),
            minimums.clone(),
            Part::Two,
        )
        .unwrap_or(9999),
        find_min_path(
            Pos(0, 1),
            &grid,
            vec![Pos(0, 0)],
            (Towards::Right, 1),
            grid.item(0, 1) as usize,
            treshold,
            minimums.clone(),
            Part::Two,
        )
        .unwrap_or(9999),
    )
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
        assert_eq!(part2_internal("res/2023/test17.txt"), 94);
    }
}
