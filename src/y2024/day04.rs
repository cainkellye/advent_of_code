use crate::utils::Grid;

use super::*;

pub fn part1() {
    println!("{:?}", part1_internal("res/2024/input04.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2024/input04.txt"));
}

const SEARCH_BYTES: &[u8] = b"XMAS";

fn part1_internal(input_file: &str) -> usize {
    let grid = Grid::new(
        iter_lines_from(input_file)
            .map(|line| line.as_bytes().to_vec())
            .collect_vec(),
    );
    //Find XMAS in all directions
    let mut counter = 0;
    for col in 0..grid.cols {
        for row in 0..grid.rows {
            counter += (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|(row_delta, col_delta)| search(&grid, row, col, *row_delta, *col_delta))
                .count();
        }
    }
    counter
}

/// row_delta: is the step for rows (e.g.: 1 for searching downwards, -1 for upwards)
/// col_delta: is the step fol cols
fn search(grid: &Grid, row: usize, col: usize, row_delta: isize, col_delta: isize) -> bool {
    for (idx, b) in SEARCH_BYTES.iter().enumerate() {
        match grid.get(
            (row as isize) + (idx as isize * row_delta),
            (col as isize) + (idx as isize * col_delta),
        ) {
            Some(byte) => {
                if &byte != b {
                    return false;
                }
            }
            None => return false,
        };
    }
    true
}

fn part2_internal(input_file: &str) -> usize {
    let grid = Grid::new(
        iter_lines_from(input_file)
            .map(|line| line.as_bytes().to_vec())
            .collect_vec(),
    );
    // Find crossed M-A-S occurances
    let mut counter = 0;
    for col in 1..grid.cols - 1 {
        for row in 1..grid.rows - 1 {
            if check_x_mas(&grid, row, col) {
                counter += 1;
            }
        }
    }
    counter
}

// Idea:
// filter though the grid, in each inner position
// check that the position is an 'A'
// check that opposing corners are 'M' and 'S'
fn check_x_mas(grid: &Grid, row: usize, col: usize) -> bool {
    if grid.get_unchecked(row, col) != b'A' {
        return false;
    }
    if !(grid.get_unchecked(row - 1, col - 1) == b'M'
        && grid.get_unchecked(row + 1, col + 1) == b'S'
        || grid.get_unchecked(row - 1, col - 1) == b'S'
            && grid.get_unchecked(row + 1, col + 1) == b'M')
    {
        return false;
    }
    if !(grid.get_unchecked(row - 1, col + 1) == b'M'
        && grid.get_unchecked(row + 1, col - 1) == b'S'
        || grid.get_unchecked(row - 1, col + 1) == b'S'
            && grid.get_unchecked(row + 1, col - 1) == b'M')
    {
        return false;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2024/input04.txt"), 166357705);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2024/input04.txt"), 88811886);
    }
}
