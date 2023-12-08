use num::Integer;
use rayon::prelude::*;
use std::collections::HashMap;

use super::*;
pub fn part1() {
    println!("{}", part1_internal("res/2023/input08.txt"));
}
pub fn part2() {
    println!("{}", part2_internal("res/2023/input08.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    let (instructions, map) = parse_input(input_file);
    let mut step_count = 0;
    let mut current = b"AAA";
    let mut instructions = instructions.into_iter().cycle();
    while current != b"ZZZ" {
        let i = instructions.next().unwrap();
        current = &map[current][i as usize];
        step_count += 1;
    }
    step_count
}

fn part2_internal(input_file: &str) -> usize {
    let (instructions, map) = parse_input(input_file);
    let instructions_length = instructions.len();
    let mut cycle_count = 0;
    let mut current = map
        .keys()
        .filter(|&&[.., x]| x == b'A' || x == b'Z')
        .map(|x| (x, Option::<usize>::None))
        .collect_vec();
    while current.par_iter().any(|(_, x)| x.is_none()) {
        // Do a complete cycle of instructions
        instructions.iter().for_each(|&i| {
            current
                .par_iter_mut()
                .filter(|(_, cycles)| cycles.is_none())
                .for_each(|(current, _)| {
                    *current = &map[*current][i as usize];
                });
        });
        cycle_count += 1;
        current
            .par_iter_mut()
            .filter(|(_, cycles)| cycles.is_none())
            .for_each(|(current, cycles)| {
                if current[2] == b'Z' {
                    *cycles = Some(cycle_count)
                }
            });
    }
    current
        .into_iter()
        .filter_map(|(_, cycles)| cycles)
        .reduce(|a, b| a.lcm(&b))
        .unwrap() * instructions_length // 16342438708751
}

type Map = HashMap<[u8; 3], [[u8; 3]; 2]>;

fn parse_input(input_file: &str) -> (Vec<u8>, Map) {
    let mut lines = iter_lines_from(input_file);
    let instructions = lines
        .next()
        .unwrap()
        .bytes()
        .map(|b| if b == b'L' { 0 } else { 1 })
        .collect_vec();
    // skip empty line
    lines.next();
    // Parse by exact positions. Example line: "TJS = (LFP, HKT)"
    let map: HashMap<[u8; 3], [[u8; 3]; 2]> = lines
        .map(|line| line.into_bytes())
        .map(|bytes| {
            (
                bytes[0..3].to_owned().try_into().unwrap(),
                [
                    bytes[7..10].to_owned().try_into().unwrap(),
                    bytes[12..15].to_owned().try_into().unwrap(),
                ],
            )
        })
        .collect();
    (instructions, map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test08.txt"), 6);
    }
}
