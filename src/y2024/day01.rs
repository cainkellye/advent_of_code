use super::*;
use std::collections::HashMap;

pub fn part1() {
    //distances
    let (mut list1, mut list2) = read_lists();
    list1.sort_unstable();
    list2.sort_unstable();
    let sum: u32 = list1
        .into_iter()
        .zip(list2)
        .map(|(x, y)| x.abs_diff(y))
        .sum();
    println!("{:?}", sum);
}

pub fn part2() {
    //frequencies
    let (list1, list2) = read_lists();
    let mut freq_map = HashMap::new();

    for &num in &list2 {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    let sum: u32 = list1
        .iter()
        .filter_map(|&n| freq_map.get(&n).map(|&count| count * n))
        .sum();

    println!("{:?}", sum);
}

fn read_lists() -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in iter_lines_from("res/2024/input01.txt") {
        if let Some((a, b)) = line.split_once("   ") {
            list1.push(a.parse::<u32>().unwrap());
            list2.push(b.parse::<u32>().unwrap());
        }
    }

    (list1, list2)
}
