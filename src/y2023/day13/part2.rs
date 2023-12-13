use super::Grid;
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
        .map(|(a, b)| {
            let mut smudge = false;
            (
                b,
                matches(&grid.row(a), &grid.row(b), &mut smudge),
                smudge,
            )
        })
        .filter(|&(_, matched, _)| matched)
        .map(|(m, _, smudge)| {
            let mut smudge = smudge;
            (m, verify_mirror_row(m, grid, &mut smudge), smudge)
        })
        .filter(|&(_, verified, smudge)| verified && smudge)
        .map(|(m, _, _)| m)
        .collect_vec();
    if mirrors.is_empty() {
        return None;
    } else if mirrors.len() == 1 {
        return Some(mirrors[0]);
    } else {
        unreachable!("More than 1 verified row");
    }
}

fn verify_mirror_row(at: usize, grid: &Grid, smudge: &mut bool) -> bool {
    let mut gap = 2;
    while at >= gap && at <= grid.rows - gap {
        if matches(grid.row(at - gap), grid.row(at + gap - 1), smudge) {
            gap += 1;
        } else {
            return false;
        }
    }
    true
}

fn find_mirror_col(grid: &Grid) -> Option<usize> {
    let mirrors = (0..grid.cols - 1)
        .zip(1..grid.cols)
        .map(|(a, b)| {
            let mut smudge = false;
            (
                b,
                matches(&grid.col(a), &grid.col(b), &mut smudge),
                smudge,
            )
        })
        .filter(|&(_, matched, _)| matched)
        .map(|(m, _, smudge)| {
            let mut smudge = smudge;
            (m, verify_mirror_col(m, grid, &mut smudge), smudge)
        })
        .filter(|&(_, verified, smudge)| verified && smudge)
        .map(|(m, _, _)| m)
        .collect_vec();
    if mirrors.is_empty() {
        return None;
    } else if mirrors.len() == 1 {
        return Some(mirrors[0]);
    } else {
        unreachable!("More than 1 verified col");
    }
}

fn verify_mirror_col(at: usize, grid: &Grid, smudge: &mut bool) -> bool {
    let mut gap = 2;
    while at >= gap && at <= grid.cols - gap {
        if matches(&grid.col(at - gap), &grid.col(at + gap - 1), smudge) {
            gap += 1;
        } else {
            return false;
        }
    }
    true
}

fn matches(left: &Vec<u8>, right: &Vec<u8>, smudge: &mut bool) -> bool {
    if left == right {
        true
    } else if !*smudge && left.iter().zip(right).filter(|(l, r)| l != r).count() == 1 {
        *smudge = true;
        true
    } else {
        false
    }
}
