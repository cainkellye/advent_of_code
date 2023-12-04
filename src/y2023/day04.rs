use super::*;
use std::collections::HashMap;

pub fn part1() {
    let sum: usize = iter_lines_from("res/2023/input04.txt")
        .map(|line| match count_wins(line) {
            0 => 0,
            n => 2_usize.pow((n - 1) as u32),
        })
        .sum();
    println!("{sum}");
}

pub fn part2() {
    let mut copies: HashMap<usize, usize> = HashMap::new();
    for (card_idx, line) in iter_lines_from("res/2023/input04.txt").enumerate() {
        copies.entry(card_idx).and_modify(|c| *c += 1).or_insert(1);
        let current_copies = *copies.get(&card_idx).unwrap();
        for win_id in 1..=count_wins(line) {
            copies
                .entry(card_idx + win_id)
                .and_modify(|c| *c += current_copies)
                .or_insert(current_copies);
        }
    }
    let sum: usize = copies.values().sum();
    println!("{sum}");
}

fn count_wins(line: String) -> usize {
    let (winners, yours) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
    let mut winner_table = [false; 100];
    for n in winners.split(' ').filter_map(|n| n.parse::<usize>().ok()) {
        winner_table[n] = true;
    }
    yours
        .split(' ')
        .filter_map(|n| n.parse::<usize>().ok())
        .filter(|&n| winner_table[n])
        .count()
}
