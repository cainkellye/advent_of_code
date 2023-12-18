use super::*;
use geo::Area;
use geo::Polygon;

pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input18.txt")); // 47675
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input18.txt")); // 122103860427465
}

fn part1_internal(input_file: &str) -> usize {
    let mut digged = vec![(0., 0.)];
    let mut position = (0., 0.);
    let mut trench = 0;
    for (direction, count, _) in parse_input(input_file) {
        match direction {
            b'U' => position.0 -= count as f64,
            b'D' => position.0 += count as f64,
            b'L' => position.1 -= count as f64,
            b'R' => position.1 += count as f64,
            _ => unreachable!(),
        }
        trench += count as usize;
        digged.push(position);
    }
    let poly = Polygon::new(digged.into(), vec![]);
    poly.unsigned_area() as usize + trench / 2 + 1
}

fn part2_internal(input_file: &str) -> usize {
    let mut digged = vec![(0., 0.)];
    let mut position = (0., 0.);
    let mut trench = 0;
    for (_, _, color_hex) in parse_input(input_file) {
        let count = usize::from_str_radix(&color_hex[1..6], 16).unwrap();
        let direction = match &color_hex[6..7] {
            "0" => b'R',
            "1" => b'D',
            "2" => b'L',
            "3" => b'U',
            _ => unreachable!(),
        };
        match direction {
            b'U' => position.0 -= count as f64,
            b'D' => position.0 += count as f64,
            b'L' => position.1 -= count as f64,
            b'R' => position.1 += count as f64,
            _ => unreachable!(),
        }
        trench += count;
        digged.push(position);
    }
    let poly = Polygon::new(digged.into(), vec![]);
    poly.unsigned_area() as usize + trench / 2 + 1
}

fn parse_input(input_file: &str) -> impl Iterator<Item = (u8, u8, String)> {
    iter_lines_from(input_file).map(|line| {
        let (direction, rest) = line.split_once(' ').unwrap();
        let (count, color) = rest.split_once(' ').unwrap();
        let color = color[1..color.len() - 1].to_owned();
        (direction.as_bytes()[0], count.parse().unwrap(), color)
    })
}
