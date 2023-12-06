use super::*;
use roots::{find_roots_quadratic, Roots::Two};

pub fn part1() {
    let races = {
        let mut lines = iter_lines_from("res/2023/input06.txt");

        let first_line = lines.next().unwrap();
        let (_, times) = first_line.split_once(":").unwrap();
        let times = times
            .split_whitespace()
            .filter_map(|t| t.parse::<usize>().ok());

        let second_line = lines.next().unwrap();
        let (_, distances) = second_line.split_once(":").unwrap();
        let distances = distances
            .split_whitespace()
            .filter_map(|t| t.parse::<usize>().ok());

        times.zip(distances).collect_vec()
    };
    let solution: usize = races
        .into_iter()
        .map(|(time, distance)| {
            let Two([low, high]) =
                find_roots_quadratic(1_f32, -1_f32 * time as f32, distance as f32)
            else {
                unreachable!()
            };
            (high.ceil() - low.ceil()) as usize
        })
        .product();
    println!("{solution}");
}

pub fn part2() {
    let (time, distance) = dbg!({
        let mut lines = iter_lines_from("res/2023/input06.txt");

        let first_line = lines.next().unwrap();
        let (_, time) = first_line.split_once(":").unwrap();
        let time = time.replace(" ", "").parse::<usize>().unwrap();

        let second_line = lines.next().unwrap();
        let (_, distance) = second_line.split_once(":").unwrap();
        let distance = distance.replace(" ", "").parse::<usize>().unwrap();
        (time, distance)
    });
    let solution: usize = {
        let Two([low, high]) = find_roots_quadratic(1_f64, -1_f64 * time as f64, distance as f64)
        else {
            unreachable!()
        };
        (high.ceil() - low.ceil()) as usize
    };
    println!("{solution}");
}
