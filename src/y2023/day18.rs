use super::*;
use geo::{Area, EuclideanLength, LineString, Polygon};

pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input18.txt")); // 47675
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input18.txt")); // 122103860427465
}

fn part1_internal(input_file: &str) -> usize {
    let mut digged = vec![(0., 0.)];
    let mut position = (0., 0.);
    for (direction, count, _) in parse_input(input_file) {
        match direction {
            b'U' => position.0 -= count as f64,
            b'D' => position.0 += count as f64,
            b'L' => position.1 -= count as f64,
            b'R' => position.1 += count as f64,
            _ => unreachable!(),
        }
        digged.push(position);
    }
    let line_string = LineString::from(digged);
    let trench = line_string.euclidean_length() as usize;
    Polygon::new(line_string, vec![]).unsigned_area() as usize + trench / 2 + 1
}

fn part2_internal(input_file: &str) -> usize {
    let mut digged = vec![(0., 0.)];
    let mut position = (0., 0.);
    for (_, _, color_hex) in parse_input(input_file) {
        let count = usize::from_str_radix(&color_hex[1..6], 16).unwrap();
        match &color_hex[6..7] {
            "0" => position.1 += count as f64,
            "1" => position.0 += count as f64,
            "2" => position.1 -= count as f64,
            "3" => position.0 -= count as f64,
            _ => unreachable!(),
        };
        digged.push(position);
    }
    let line_string = LineString::from(digged);
    let trench = line_string.euclidean_length() as usize;
    Polygon::new(line_string, vec![]).unsigned_area() as usize + trench / 2 + 1
}

fn parse_input(input_file: &str) -> impl Iterator<Item = (u8, u8, String)> {
    iter_lines_from(input_file).map(|line| {
        let (direction, rest) = line.split_once(' ').unwrap();
        let (count, color) = rest.split_once(' ').unwrap();
        let color = color[1..color.len() - 1].to_owned();
        (direction.as_bytes()[0], count.parse().unwrap(), color)
    })
}
