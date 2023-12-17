use super::*;
use crate::utils::Part;
use std::cmp::Ordering;

pub fn part1() {
    println!("{}", solve(Part::One)); //250602641
}
pub fn part2() {
    println!("{}", solve(Part::Two)); //251037509
}

fn solve(p: Part) -> usize {
    iter_lines_from("res/2023/input07.txt")
        .filter_map(|line| {
            line.split_once(' ')
                .map(|(a, b)| (a.to_owned(), b.to_owned()))
        })
        .sorted_unstable_by(|(a, _), (b, _)| compare_hands(a, b, p))
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid.parse::<usize>().unwrap())
        .sum()
}

fn compare_hands(a: &str, b: &str, p: Part) -> Ordering {
    let cmp = score_hand(a, p).cmp(&score_hand(b, p));
    if cmp != Ordering::Equal {
        return cmp;
    }
    let (a, b) = a
        .chars()
        .zip(b.chars())
        .find(|(a, b)| a != b)
        .expect("totally same hands");
    face_value(a, p).cmp(&face_value(b, p))
}

fn score_hand(hand: &str, p: Part) -> [u8; 2] {
    let mut faces = [0_u8; 13];
    let mut jokers = 0;
    for c in hand.chars() {
        faces[face_value(c, p)] += 1;
    }
    if p == Part::Two {
        jokers = faces[0];
        faces[0] = 0;
    }
    faces.sort_unstable();
    let mut score: [u8; 2] = faces[11..].try_into().unwrap();
    score.reverse();
    score[0] += jokers;
    score
}

fn face_value(c: char, p: Part) -> usize {
    if p == Part::One {
        b"23456789TJQKA"
    } else {
        b"J23456789TQKA"
    }
    .iter()
    .position(|&face| face == c as u8)
    .unwrap()
}
