use crate::utils::Grid;
use rayon::prelude::*;
use std::{cell::RefCell, rc::Rc, sync::Arc};

use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input16.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input16.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    let grid = Arc::new(parse_input(input_file));
    count_energized(0, 0, Towards::Right, grid)
}

fn part2_internal(input_file: &str) -> usize {
    let grid = Arc::new(parse_input(input_file));
    (0..grid.rows)
        .cartesian_product(0..grid.cols)
        .par_bridge()
        .map(|(r, c)| match (r, c) {
            (0, 0) => usize::max(
                count_energized(0, 0, Towards::Right, grid.clone()),
                count_energized(0, 0, Towards::Down, grid.clone()),
            ),
            (0, c) if c == grid.cols - 1 => usize::max(
                count_energized(0, c, Towards::Left, grid.clone()),
                count_energized(0, c, Towards::Down, grid.clone()),
            ),
            (r, 0) if r == grid.rows - 1 => usize::max(
                count_energized(r, 0, Towards::Right, grid.clone()),
                count_energized(r, 0, Towards::Up, grid.clone()),
            ),
            (r, c) if r == grid.rows - 1 && c == grid.cols - 1 => usize::max(
                count_energized(r, c, Towards::Left, grid.clone()),
                count_energized(r, c, Towards::Up, grid.clone()),
            ),
            (0, c) => count_energized(0, c, Towards::Down, grid.clone()),
            (r, c) if r == grid.rows - 1 => count_energized(r, c, Towards::Up, grid.clone()),
            (r, 0) => count_energized(r, 0, Towards::Right, grid.clone()),
            (r, c) if c == grid.cols - 1 => count_energized(r, c, Towards::Left, grid.clone()),
            _ => 0,
        })
        .max()
        .unwrap()
}

type TravelledDirections = Vec<Towards>;
type TraceMatrix = Rc<RefCell<Vec<Vec<TravelledDirections>>>>;

#[rustfmt::skip]
fn count_energized(row: usize, col: usize, direction: Towards, grid: Arc<Grid>) -> usize {
    let energized: TraceMatrix = Rc::new(RefCell::new(vec![vec![vec![]; grid.cols]; grid.rows]));
    trace_light(LightState { row, col, direction, }, grid, energized.clone());
    let energized = energized.borrow();
    energized
        .iter()
        .map(|row| row.iter().filter(|col| !col.is_empty()).count())
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Towards {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct LightState {
    row: usize,
    col: usize,
    direction: Towards,
}

#[rustfmt::skip]
fn trace_light(mut light: LightState, grid: Arc<Grid>, energized: TraceMatrix) {
    loop {
        if energized.borrow()[light.row][light.col].contains(&light.direction) {
            break;
        }
        energized.borrow_mut()[light.row][light.col].push(light.direction);
        light.direction = match (grid.item(light.row, light.col), light.direction) {
            (b'.', direction) => direction,

            (b'/', Towards::Right) => Towards::Up,
            (b'/', Towards::Down) => Towards::Left,
            (b'/', Towards::Left) => Towards::Down,
            (b'/', Towards::Up) => Towards::Right,

            (b'\\', Towards::Right) => Towards::Down,
            (b'\\', Towards::Down) => Towards::Right,
            (b'\\', Towards::Left) => Towards::Up,
            (b'\\', Towards::Up) => Towards::Left,

            (b'-', Towards::Left | Towards::Right) => light.direction,
            (b'|', Towards::Up | Towards::Down) => light.direction,

            (b'-', Towards::Up | Towards::Down) => {
                trace_light(LightState { direction: Towards::Left, ..light }, grid.clone(), energized.clone());
                Towards::Right
            }
            (b'|', Towards::Left | Towards::Right) => {
                trace_light(LightState { direction: Towards::Up, ..light }, grid.clone(), energized.clone());
                Towards::Down
            }

            _ => unreachable!(),
        };
        match light.direction {
            Towards::Up => { if light.row == 0 { break; } else { light.row -= 1 } }
            Towards::Down => { if light.row == grid.rows - 1 { break; } else { light.row += 1 } }
            Towards::Left => { if light.col == 0 { break; } else { light.col -= 1 } }
            Towards::Right => { if light.col == grid.cols - 1 { break; } else { light.col += 1 } }
        }
    }
}

fn parse_input(input_file: &str) -> Grid {
    Grid::new(
        iter_lines_from(input_file)
            .map(|line| line.as_bytes().to_vec())
            .collect_vec(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test16.txt"), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test16.txt"), 51);
    }
}
