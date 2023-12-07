use super::*;
use roots::{find_roots_quadratic, Roots::Two};

/// Reasoning
/// --------------
/// t = time limit; p = push time; d = distance
/// d = (t-p) * p
/// Boat wins if p satisfies:
/// 0 < tp - pp - d
/// pp - tp + d < 0
pub fn part1() {
    let (times, distances) = iter_lines_from("res/2023/input06.txt")
        .filter_map(|line| line.split_once(':').map(|(_, x)| x.to_owned()))
        .map(|numbers| {
            numbers
                .split_whitespace()
                .filter_map(|n| n.parse::<f64>().ok())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    let solution: usize = times.into_iter().zip(distances)
        .map(|(time, distance)| {
            if let Two([low, high]) =
                find_roots_quadratic(1_f64, -time, distance)
            {
                (high.ceil() - low.ceil()) as usize
            } else {
                1
            }
        }).product();
    println!("{solution}");
}

pub fn part2() {
    let (time, distance) = iter_lines_from("res/2023/input06.txt")
        .filter_map(|line| line.split_once(':').map(|(_, x)| x.to_owned()))
        .filter_map(|number| number.replace(' ', "").parse::<f64>().ok())
        .collect_tuple()
        .unwrap();

    let solution: usize = if let Two([low, high]) =
        find_roots_quadratic(1_f64, -time, distance)
    {
        (high.ceil() - low.ceil()) as usize
    } else {
        1
    };
    println!("{solution}");
}
