use super::*;
use std::collections::HashMap;

pub fn part1() {
    let sum: usize = iter_lines_from("res/2023/input04.txt")
        .map(|line| {
            let (winners, yours) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let mut w = [false; 100];
            for n in winners.split(' ').filter(|n| !n.is_empty()) {
                w[n.parse::<usize>().unwrap()] = true;
            }
            let mut exp: i32 = -1;
            for n in yours.split(' ').filter(|n| !n.is_empty()) {
                if w[n.parse::<usize>().unwrap()] {
                    exp += 1;
                }
            }
            if exp >= 0 {
                2_usize.pow(exp as u32)
            } else {
                0
            }
        })
        .sum();
    println!("{sum}");
}

pub fn part2() {
    let mut copies: HashMap<usize, usize> = HashMap::new();
    for (card_idx, line) in iter_lines_from("res/2023/input04.txt").enumerate() {
        copies.entry(card_idx).and_modify(|c| *c += 1).or_insert(1);
        let (winners, yours) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let mut w = [false; 100];
        for n in winners.split(' ').filter(|n| !n.is_empty()) {
            w[n.parse::<usize>().unwrap()] = true;
        }
        let mut wins: usize = 0;
        for n in yours.split(' ').filter(|n| !n.is_empty()) {
            if w[n.parse::<usize>().unwrap()] {
                wins += 1;
            }
        }
        let current_copies = *copies.get(&card_idx).unwrap();
        for win in 1..=wins {
            copies
                .entry(card_idx + win)
                .and_modify(|c| *c += current_copies)
                .or_insert(current_copies);
        }
    }
    let sum: usize = copies.values().sum();
    println!("{sum}");
}
