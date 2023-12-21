use crate::utils::{Grid, Part};
use std::collections::{HashMap, VecDeque};

use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input17.txt")); // 956
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input17.txt")); // 1106
}

struct Step(Towards, u8, Pos, usize);

fn part1_internal(input_file: &str) -> usize {
    let grid = parse_input(input_file);
    let mut step_queue = VecDeque::new();
    step_queue.push_back(Step(Towards::Down, 1, Pos(1, 0), grid.item(1, 0) as usize));
    step_queue.push_back(Step(Towards::Right, 1, Pos(0, 1), grid.item(0, 1) as usize));
    find_min_path(step_queue, &grid, Part::One)
}

fn part2_internal(input_file: &str) -> usize {
    let grid = parse_input(input_file);
    let mut step_queue = VecDeque::new();
    step_queue.push_back(Step(Towards::Down, 1, Pos(1, 0), grid.item(1, 0) as usize));
    step_queue.push_back(Step(Towards::Right, 1, Pos(0, 1), grid.item(0, 1) as usize));
    find_min_path(step_queue, &grid, Part::Two)
}

type BestSteps = HashMap<(Towards, u8, Pos), usize>;

fn find_min_path(mut step_queue: VecDeque<Step>, grid: &Grid, part: Part) -> usize {
    let bounds = Pos(grid.rows - 1, grid.cols - 1);
    let mut best_steps: BestSteps = HashMap::new();
    step_queue
        .make_contiguous()
        .sort_by_key(|&Step(_, _, _, weight)| weight);
    let mut paths_weight = 99999;
    while let Some(Step(direction, count, pos, weight)) = step_queue.pop_front() {
        if paths_weight < weight {
            continue;
        }
        if let Some(best) = best_steps.get_mut(&(direction, count, pos)) {
            if weight < *best {
                *best = weight;
            } else {
                continue;
            }
        } else {
            best_steps.insert((direction, count, pos), weight);
        }
        if pos == bounds {
            if part == Part::Two && count < 4 {
                continue;
            }
            paths_weight = weight;
            continue;
        }
        let next_steps = get_valid_steps(Step(direction, count, pos, weight), grid, &bounds, part);
        for next in next_steps {
            let pos = step_queue
                .binary_search_by_key(&next.3, |&Step(_, _, _, weight)| weight)
                .unwrap_or_else(|x| x);
            step_queue.insert(pos, next);
        }
    }
    paths_weight
}

fn get_valid_steps(from: Step, grid: &Grid, bounds: &Pos, part: Part) -> Vec<Step> {
    let mut valid = vec![];
    let Step(last_direction, dir_count, pos, weight) = from;
    if part == Part::One {
        if dir_count < 3 {
            if let Some(new_pos) = pos.step(last_direction, bounds) {
                valid.push(Step(
                    last_direction,
                    dir_count + 1,
                    new_pos,
                    weight + grid.item(new_pos.0, new_pos.1) as usize,
                ));
            }
        }
        for direction in [
            last_direction.clockwise(),
            last_direction.counter_clockwise(),
        ] {
            if let Some(new_pos) = pos.step(direction, bounds) {
                valid.push(Step(
                    direction,
                    1,
                    new_pos,
                    weight + grid.item(new_pos.0, new_pos.1) as usize,
                ));
            }
        }
    } else {
        if dir_count < 10 {
            if let Some(new_pos) = pos.step(last_direction, bounds) {
                valid.push(Step(
                    last_direction,
                    dir_count + 1,
                    new_pos,
                    weight + grid.item(new_pos.0, new_pos.1) as usize,
                ));
            }
        }
        if dir_count >= 4 {
            for direction in [
                last_direction.clockwise(),
                last_direction.counter_clockwise(),
            ] {
                if let Some(new_pos) = pos.step(direction, bounds) {
                    valid.push(Step(
                        direction,
                        1,
                        new_pos,
                        weight + grid.item(new_pos.0, new_pos.1) as usize,
                    ));
                }
            }
        }
    }
    valid
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
