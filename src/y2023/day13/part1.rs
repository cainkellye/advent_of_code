use crate::utils::Grid;
use itertools::Itertools;

pub fn find_mirror(grid: &Grid) -> usize {
    let row = find_mirror_row(grid);
    let col = find_mirror_col(grid);
    match (row, col) {
        (Some(row), Some(col)) => {
            unreachable!("Both row and col found: {:?} {:?}\n{:?}", row, col, grid)
        }
        (None, Some(col)) => col,
        (Some(row), None) => row * 100,
        (None, None) => {
            unreachable!("No mirror found {:?}", grid);
        }
    }
}

fn find_mirror_row(grid: &Grid) -> Option<usize> {
    let mirrors = (0..grid.rows - 1)
        .zip(1..grid.rows)
        .filter(|&(a, b)| grid.row(a) == grid.row(b))
        .map(|(_, b)| b)
        .filter(|&m| verify_mirror_row(m, grid))
        .collect_vec();
    if mirrors.is_empty() {
        None
    } else if mirrors.len() == 1 {
        Some(mirrors[0])
    } else {
        unreachable!("More than 1 verified row");
    }
}

fn verify_mirror_row(at: usize, grid: &Grid) -> bool {
    let mut gap = 2;
    while at >= gap && at <= grid.rows - gap {
        if grid.row(at - gap) != grid.row(at + gap - 1) {
            return false;
        }
        gap += 1;
    }
    true
}

fn find_mirror_col(grid: &Grid) -> Option<usize> {
    let mirrors = (0..grid.cols - 1)
        .zip(1..grid.cols)
        .filter(|&(a, b)| grid.col(a) == grid.col(b))
        .map(|(_, b)| b)
        .filter(|&m| verify_mirror_col(m, grid))
        .collect_vec();
    if mirrors.is_empty() {
        None
    } else if mirrors.len() == 1 {
        Some(mirrors[0])
    } else {
        unreachable!("More than 1 verified col");
    }
}

fn verify_mirror_col(at: usize, grid: &Grid) -> bool {
    let mut gap = 2;
    while at >= gap && at <= grid.cols - gap {
        if grid.col(at - gap) != grid.col(at + gap - 1) {
            return false;
        }
        gap += 1;
    }
    true
}
