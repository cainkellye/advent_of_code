use std::fmt::{Debug, Write};

use rayon::iter::IndexedParallelIterator;

use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input13.txt")); // 33975
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input13.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    parse_input(input_file)
        .into_iter()
        .map(|grid| find_mirror(&grid))
        .sum()
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file);
    0
}

struct Grid {
    grid: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n').unwrap();
        f.write_str(
            &self
                .grid
                .iter()
                .map(|line| String::from_utf8_lossy(line))
                .join("\n"),
        )
    }
}

impl Grid {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        Self {
            rows: grid.len(),
            cols: grid[0].len(),
            grid,
        }
    }
    fn row(&self, index: usize) -> &Vec<u8> {
        &self.grid[index]
    }
    fn col(&self, index: usize) -> Vec<u8> {
        self.grid.iter().map(|row| row[index]).collect_vec()
    }
}

fn find_mirror(grid: &Grid) -> usize {
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
        return None;
    } else if mirrors.len() == 1 {
        return Some(mirrors[0]);
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
        return None;
    } else if mirrors.len() == 1 {
        return Some(mirrors[0]);
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

fn parse_input(input_file: &str) -> Vec<Grid> {
    iter_lines_from(input_file)
        .batching(|lines| {
            let grid = lines
                .take_while(|line| !line.is_empty())
                .map(|line| line.as_bytes().to_vec())
                .collect_vec();
            if grid.is_empty() {
                None
            } else {
                Some(Grid::new(grid))
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let grid1 = Grid::new(vec![
            b"#.##..##.".to_vec(),
            b"..#.##.#.".to_vec(),
            b"##......#".to_vec(),
            b"##......#".to_vec(),
            b"..#.##.#.".to_vec(),
            b"..##..##.".to_vec(),
            b"#.#.##.#.".to_vec(),
        ]);

        let grid2 = Grid::new(vec![
            b"#...##..#".to_vec(),
            b"#....#..#".to_vec(),
            b"..##..###".to_vec(),
            b"#####.##.".to_vec(),
            b"#####.##.".to_vec(),
            b"..##..###".to_vec(),
            b"#....#..#".to_vec(),
        ]);

        let grid3 = Grid::new(vec![
            b".......##..##".to_vec(),
            b"..##...#.#...".to_vec(),
            b".#..#...#.###".to_vec(),
            b".####...#..##".to_vec(),
            b"##..##.####.#".to_vec(),
            b"#.##.#.#.....".to_vec(),
            b".......#.##..".to_vec(),
            b"..##..#.#.###".to_vec(),
            b"......#..#..#".to_vec(),
            b".......#.##.#".to_vec(),
            b"......#..#.##".to_vec(),
            b"#...##..#..##".to_vec(),
            b".####.#.#..##".to_vec(),
            b".####.#.##...".to_vec(),
            b".####.#.##...".to_vec(),
        ]);

        let grid4 = Grid::new(vec![
            b"...##.#.#..##".to_vec(),
            b".#.#..#.#..##".to_vec(),
            b".##.##...#..#".to_vec(),
            b".##.##...#..#".to_vec(),
            b".#.#..#.#..##".to_vec(),
            b"...##.#.#..##".to_vec(),
            b".#.#.##....#.".to_vec(),
            b"########....#".to_vec(),
            b"##..#.#...#.#".to_vec(),
            b"......##.....".to_vec(),
            b"#..#.....###.".to_vec(),
            b"#.#.#####.##.".to_vec(),
            b"#.#.#####.##.".to_vec(),
            b"#..#.....###.".to_vec(),
            b"......##.....".to_vec(),
            b"##..#.#.#.#.#".to_vec(),
            b"########....#".to_vec(),
        ]);

        assert_eq!(find_mirror(&grid1), 5);
        assert_eq!(find_mirror(&grid2), 400);
        assert_eq!(find_mirror(&grid3), 1400);
        assert_eq!(find_mirror(&grid4), 300);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test13.txt"), 0);
    }
}
