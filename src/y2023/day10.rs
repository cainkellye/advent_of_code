use std::{default, io::stdin};

use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input10.txt")); // 6778
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input10.txt")); // 433
}

type Coord = (i32, i32);
type Pipe = [Coord; 2];
type Grid = Vec<Vec<Pipe>>;

fn part1_internal(input_file: &str) -> usize {
    traverse(parse_input(input_file)).0
}

#[allow(clippy::needless_range_loop)] // need to be explicit to be able to modify the grid in the loop
fn part2_internal(input_file: &str) -> usize {
    let (grid, start) = parse_input(input_file);
    let (_, main_loop, start_connections) = traverse((grid.clone(), start));
    let mut grid_b = parse_grid(input_file);
    let (rows, cols) = (grid_b.len(), grid_b[0].len());

    grid_b[start.0 as usize][start.1 as usize] = get_start_symbol(start, start_connections);

    let mut count = 0;
    for row in 0..rows {
        let mut inside = false;
        let mut last = b'.';
        for col in 0..cols {
            match grid_b[row][col] {
                s if main_loop.contains(&(row as i32, col as i32)) => {
                    if s == b'|'
                        || s == b'F'
                        || s == b'L'
                        || (s == b'J' && last == b'L')
                        || (s == b'7' && last == b'F')
                    {
                        last = s;
                        inside ^= true;
                        grid_b[row][col] = b'|';
                    } else {
                        grid_b[row][col] = b'#';
                    }
                }
                _ if inside => {
                    grid_b[row][col] = b'o';
                    count += 1;
                }
                _ => grid_b[row][col] = b'.',
            }
        }
    }
    let visualize = false;
    if visualize {
        for row in 0..rows {
            for col in 0..cols {
                print!("{}", grid_b[row][col] as char);
            }
            println!();
        }
    }
    count
}

fn get_start_symbol(start_coord: Coord, start_pipe: Pipe) -> u8 {
    let left = (start_coord.0, start_coord.1 - 1);
    let right = (start_coord.0, start_coord.1 + 1);
    let up = (start_coord.0 - 1, start_coord.1);
    let down = (start_coord.0 + 1, start_coord.1);
    if start_pipe.contains(&left) && start_pipe.contains(&right) {
        b'-'
    } else if start_pipe.contains(&left) && start_pipe.contains(&up) {
        b'J'
    } else if start_pipe.contains(&left) && start_pipe.contains(&down) {
        b'7'
    } else if start_pipe.contains(&right) && start_pipe.contains(&up) {
        b'L'
    } else if start_pipe.contains(&right) && start_pipe.contains(&down) {
        b'F'
    } else {
        b'|'
    }
}

fn traverse((mut grid, start): (Grid, Coord)) -> (usize, Vec<Coord>, Pipe) {
    let start_neighbours = get_start_neighbours(start, &grid);

    // The input data has only 2 connections to start
    if cfg!(debug_assertions) {
        println!("{:?}", start_neighbours);
        assert_eq!(start_neighbours.len(), 2);
    }

    let mut prev_a: Coord = start_neighbours[0];
    let mut prev_b: Coord = start_neighbours[1];
    grid[start.0 as usize][start.1 as usize] = [prev_a, prev_b];
    let mut current_a: Coord = get_next_coord(start_neighbours[0], start, &grid);
    let mut current_b: Coord = get_next_coord(start_neighbours[1], start, &grid);

    let mut touched: Vec<Coord> = vec![start, prev_a, prev_b, current_a, current_b];

    for step in 2.. {
        if current_a == current_b {
            return (step, touched, grid[start.0 as usize][start.1 as usize]);
        }
        (current_a, prev_a) = (get_next_coord(current_a, prev_a, &grid), current_a);
        (current_b, prev_b) = (get_next_coord(current_b, prev_b, &grid), current_b);

        touched.push(current_a);
        touched.push(current_b);
    }
    unreachable!()
}

fn get_start_neighbours((start_row, start_col): Coord, grid: &Grid) -> Vec<Coord> {
    [
        (start_row - 1, start_col),
        (start_row + 1, start_col),
        (start_row, start_col - 1),
        (start_row, start_col + 1),
    ]
    .into_iter()
    .filter(|&(row, col)| {
        row >= 0 && row < grid.len() as i32 && col >= 0 && col < grid[0].len() as i32
    })
    .filter(|&(row, col)| grid[row as usize][col as usize].contains(&(start_row, start_col)))
    .collect_vec()
}

fn get_next_coord(current: Coord, prev: Coord, grid: &Grid) -> Coord {
    *grid[current.0 as usize][current.1 as usize]
        .iter()
        .find(|&&coord| coord != prev)
        .unwrap()
}

fn parse_input(input_file: &str) -> (Grid, Coord) {
    let mut start = Default::default();
    let grid = iter_lines_from(input_file)
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, symbol)| {
                    let (row, col) = (row as i32, col as i32);
                    match symbol {
                        b'|' => [(row - 1, col), (row + 1, col)], // is a vertical pipe connecting north and south.
                        b'-' => [(row, col - 1), (row, col + 1)], // is a horizontal pipe connecting east and west.
                        b'L' => [(row - 1, col), (row, col + 1)], // is a 90-degree bend connecting north and east.
                        b'J' => [(row - 1, col), (row, col - 1)], // is a 90-degree bend connecting north and west.
                        b'7' => [(row + 1, col), (row, col - 1)], // is a 90-degree bend connecting south and west.
                        b'F' => [(row + 1, col), (row, col + 1)], // is a 90-degree bend connecting south and east.
                        b'.' => [(255, 255), (255, 255)], // is ground; there is no pipe in this tile.
                        b'S' => {
                            // is the starting position of the animal;
                            start = (row, col);
                            [(255, 255), (255, 255)] //we don't know it's connections yet
                        }
                        _ => unreachable!(),
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    (grid, start)
}

fn parse_grid(input_file: &str) -> Vec<Vec<u8>> {
    iter_lines_from(input_file)
        .map(|line| line.bytes().collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test10-1.txt"), 4);
        assert_eq!(part1_internal("res/2023/test10-2.txt"), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test10-3.txt"), 8);
        assert_eq!(part2_internal("res/2023/test10-4.txt"), 10);
    }
}
