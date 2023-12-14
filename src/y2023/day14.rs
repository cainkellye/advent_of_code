use crate::utils::Grid;
use std::cmp::Reverse;
use std::collections::VecDeque;

use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input14.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input14.txt")); // 89048 < ? < 89133
}

fn part1_internal(input_file: &str) -> usize {
    let mut grid = parse_input(input_file).transposed();
    for row in grid.buffer.iter_mut() {
        row.split_mut(|c| c == &b'#')
            .for_each(|part| part.sort_unstable_by_key(|x| Reverse(*x)));
    }
    let grid = grid.transposed();
    load_on_north(grid)
}

fn load_on_north(grid: Grid) -> usize {
    grid.buffer
        .iter()
        .enumerate()
        .map(|(id, row)| (grid.rows - id) * row.iter().filter(|&&b| b == b'O').count())
        .sum()
}

fn part2_internal(input_file: &str) -> usize {
    let mut cache = VecDeque::new();
    let mut grid = parse_input(input_file);
    let target_cycles = 1_000_000_000;
    let mut cycle_count = 0;
    while cycle_count < target_cycles {
        grid = cycle(grid);
        cycle_count += 1;

        if let Some(idx) = cache.iter().position(|p| p == &grid) {
            let repeat_length = cache.len() - idx;
            //time jump
            cycle_count = target_cycles - ((target_cycles - cycle_count) % repeat_length);
        }

        cache.push_back(grid.clone());
        if cache.len() > 50 {
            cache.pop_front();
        }
    }
    load_on_north(grid)
}

///  ⇑   ⇐   ⇓   ⇒
fn cycle(mut grid: Grid) -> Grid {
    for _ in 0..2 {
        grid = grid.transposed(); // swap ⇑ and ⇐
        grid.buffer.iter_mut().for_each(|row| {
            row.split_mut(|c| c == &b'#')
                .for_each(|part| part.sort_unstable_by_key(|x| Reverse(*x)))
        });
    }
    for _ in 0..2 {
        grid = grid.transposed(); // swap ⇓ and ⇒
        grid.buffer.iter_mut().for_each(|row| {
            row.split_mut(|c| c == &b'#')
                .for_each(|part| part.sort_unstable())
        });
    }
    grid
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
        assert_eq!(part1_internal("res/2023/test14.txt"), 136);
    }

    #[test]
    fn test_part2() {
        let mut grid = parse_input("res/2023/test14.txt");
        let after1 = Grid::new(vec![
            b".....#....".to_vec(),
            b"....#...O#".to_vec(),
            b"...OO##...".to_vec(),
            b".OO#......".to_vec(),
            b".....OOO#.".to_vec(),
            b".O#...O#.#".to_vec(),
            b"....O#....".to_vec(),
            b"......OOOO".to_vec(),
            b"#...O###..".to_vec(),
            b"#..OO#....".to_vec(),
        ]);
        grid = cycle(grid);
        assert_eq!(grid, after1);

        let after2 = Grid::new(vec![
            b".....#....".to_vec(),
            b"....#...O#".to_vec(),
            b".....##...".to_vec(),
            b"..O#......".to_vec(),
            b".....OOO#.".to_vec(),
            b".O#...O#.#".to_vec(),
            b"....O#...O".to_vec(),
            b".......OOO".to_vec(),
            b"#..OO###..".to_vec(),
            b"#.OOO#...O".to_vec(),
        ]);
        grid = cycle(grid);
        assert_eq!(grid, after2);

        assert_eq!(part2_internal("res/2023/test14.txt"), 64);
    }
}
