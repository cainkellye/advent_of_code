use super::*;
use num::Integer;
use std::collections::HashMap;

pub fn part1() {
    println!("{}", part1_internal("res/2023/input08.txt"));
}
pub fn part2() {
    println!("{}", part2_internal("res/2023/input08.txt"));
}

type Map = HashMap<[u8; 3], [[u8; 3]; 2]>;

fn part1_internal(input_file: &str) -> usize {
    let (instructions, map) = parse_input(input_file);
    let mut step_count = 0;
    let mut current = b"AAA";
    let mut instructions = instructions.into_iter().cycle();
    while current != b"ZZZ" {
        let i = instructions.next().unwrap();
        current = &map[current][i];
        step_count += 1;
    }
    step_count
}

fn part2_internal(input_file: &str) -> usize {
    let (instructions, map) = parse_input(input_file);
    let mut cycle_count = 0;
    let mut current = map
        .keys()
        .filter(|&&[.., x]| x == b'A')
        .map(|x| (x, Option::<usize>::None))
        .collect_vec();
    while current.iter().any(|(_, x)| x.is_none()) {
        // Do a complete cycle of instructions
        instructions.iter().for_each(|&i| {
            current
                .iter_mut()
                .filter(|(_, cycles)| cycles.is_none())
                .for_each(|(current, _)| {
                    *current = &map[*current][i];
                });
        });
        cycle_count += 1;
        current
            .iter_mut()
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
        .unwrap()
        * instructions.len() // 16342438708751
}

fn parse_input(input_file: &str) -> (Vec<usize>, Map) {
    let mut lines = iter_lines_from(input_file);
    let instructions = lines
        .next()
        .unwrap()
        .bytes()
        .map(|b| if b == b'L' { 0 } else { 1 })
        .collect_vec();
    // skip empty line
    lines.next();
    // Parse by exact positions. Sample line: "TJS = (LFP, HKT)"
    let map: HashMap<[u8; 3], [[u8; 3]; 2]> = lines
        .map(|line| line.into_bytes())
        .map(|bytes| {
            (
                bytes[0..3].try_into().unwrap(),
                [
                    bytes[7..10].try_into().unwrap(),
                    bytes[12..15].try_into().unwrap(),
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
