use rayon::iter::IndexedParallelIterator;

use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input13.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input13.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    parse_input(input_file).into_iter().map(|grid| find_mirror(&grid)).sum()
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file);
    0
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
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
    if let Some(row) = find_mirror_row(grid) {
        return row;
    }
    if let Some(col) = find_mirror_col(grid) {
        return col * 100;
    }
    unreachable!("No mirror found {:?}", grid);
}

fn find_mirror_row(grid: &Grid) -> Option<usize> {
    let mirror_pairs = (0..grid.rows)
        .tuple_windows()
        .filter(|&(a, b)| grid.row(a) == grid.row(b))
        .collect_vec();
    if mirror_pairs.is_empty() {
        return None;
    } else if mirror_pairs.len() == 1 {
        return Some(mirror_pairs[0].1);
    } else {
        todo!("broaden search");
    }
}

fn find_mirror_col(grid: &Grid) -> Option<usize> {
    let mirror_pairs = (0..grid.cols)
        .tuple_windows()
        .filter(|&(a, b)| grid.col(a) == grid.col(b))
        .collect_vec();
    if mirror_pairs.is_empty() {
        return None;
    } else if mirror_pairs.len() == 1 {
        return Some(mirror_pairs[0].1);
    } else {
        todo!("broaden search");
    }
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
        assert_eq!(part1_internal("res/2023/test13.txt"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test13.txt"), 0);
    }
}
