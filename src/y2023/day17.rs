use crate::utils::{Grid, Part};
use crate::utils::pathfinder::{Map, Step};

use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input17.txt")); // 956
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input17.txt")); // 1106
}

fn part1_internal(input_file: &str) -> usize {
    let grid = parse_input(input_file);
    let start = [
        Step {
            data: StepData(Towards::Down, 1, Pos(1, 0)),
            weight: grid.item(1, 0) as usize,
        },
        Step {
            data: StepData(Towards::Right, 1, Pos(0, 1)),
            weight: grid.item(0, 1) as usize,
        },
    ];
    let bounds = Pos(grid.rows - 1, grid.cols - 1);
    let pathfinder = Pathfinder(Part::One, grid);
    pathfinder.find_min_weight(
        start,
        |Step {
             data: StepData(_, _, pos),
             ..
         }| pos == bounds,
    )
}

fn part2_internal(input_file: &str) -> usize {
    let grid = parse_input(input_file);
    let start = [
        Step {
            data: StepData(Towards::Down, 1, Pos(1, 0)),
            weight: grid.item(1, 0) as usize,
        },
        Step {
            data: StepData(Towards::Right, 1, Pos(0, 1)),
            weight: grid.item(0, 1) as usize,
        },
    ];
    let bounds = Pos(grid.rows - 1, grid.cols - 1);
    let pathfinder = Pathfinder(Part::Two, grid);
    pathfinder.find_min_weight(
        start,
        |Step {
             data: StepData(_, count, pos),
             ..
         }| pos == bounds && count >= 4,
    )
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct StepData(Towards, u8, Pos);

struct Pathfinder(Part, Grid);
impl Map<StepData, usize> for Pathfinder {
    fn get_valid_steps(&self, from: Step<StepData, usize>) -> Vec<Step<StepData, usize>> {
        let grid = &self.1;
        let bounds = &Pos(self.1.rows - 1, self.1.cols - 1);
        let mut valid = vec![];
        let Step {
            data: StepData(last_direction, dir_count, pos),
            weight,
        } = from;
        if self.0 == Part::One {
            if dir_count < 3 {
                if let Some(new_pos) = pos.step(last_direction, bounds) {
                    valid.push(Step {
                        data: StepData(last_direction, dir_count + 1, new_pos),
                        weight: weight + grid.item(new_pos.0, new_pos.1) as usize,
                    });
                }
            }
            for direction in [
                last_direction.clockwise(),
                last_direction.counter_clockwise(),
            ] {
                if let Some(new_pos) = pos.step(direction, bounds) {
                    valid.push(Step {
                        data: StepData(direction, 1, new_pos),
                        weight: weight + grid.item(new_pos.0, new_pos.1) as usize,
                    });
                }
            }
        } else {
            if dir_count < 10 {
                if let Some(new_pos) = pos.step(last_direction, bounds) {
                    valid.push(Step {
                        data: StepData(last_direction, dir_count + 1, new_pos),
                        weight: weight + grid.item(new_pos.0, new_pos.1) as usize,
                    });
                }
            }
            if dir_count >= 4 {
                for direction in [
                    last_direction.clockwise(),
                    last_direction.counter_clockwise(),
                ] {
                    if let Some(new_pos) = pos.step(direction, bounds) {
                        valid.push(Step {
                            data: StepData(direction, 1, new_pos),
                            weight: weight + grid.item(new_pos.0, new_pos.1) as usize,
                        });
                    }
                }
            }
        }
        valid
    }

    fn get_cutoff() -> usize {
        99999
    }
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
