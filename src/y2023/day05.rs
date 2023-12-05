#![allow(unused)]
use super::*;
use rayon::prelude::*;

pub fn part1() {
    let (seeds, maps) = parse_input();
    let locations = seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |value, map| map.translate(value)));
    let min = locations.min().unwrap();
    println!("{min}");
}

pub fn part2() {
    let (seeds, maps) = parse_input();
    let locations = seeds
        .into_par_iter()
        .chunks(2)
        .flat_map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
        .map(|seed| maps.iter().fold(seed, |value, map| map.translate(value)));
    let min = locations.min().unwrap();
    println!("{min}");
}

/// dest, source, range
#[derive(Debug)]
struct Map(Vec<(usize, usize, usize)>);
impl Map {
    fn new() -> Self {
        Map(vec![])
    }
    fn add(&mut self, line: &str) {
        self.0.push(
            line.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap(),
        );
    }
    fn sort(&mut self) {
        self.0.sort_unstable_by_key(|&(dest, source, range)| source);
    }
    fn translate(&self, value: usize) -> usize {
        let mapping = self
            .0
            .iter()
            .find(|&&(dest, source, range)| source <= value && value < source + range);
        if let Some((dest, source, range)) = mapping {
            dest + value - source
        } else {
            value
        }
    }
}

fn parse_input() -> (Vec<usize>, Vec<Map>) {
    let mut maps = Vec::<Map>::new();
    let mut lines = iter_lines_from("res/2023/input05.txt");
    let first_line = lines.next().unwrap();
    let (_, seeds) = first_line.split_once(": ").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();

    for line in lines.filter(|l| !l.is_empty()) {
        if line.chars().nth(0).unwrap().is_alphabetic() {
            if let Some(map) = maps.last_mut() {
                map.sort();
            }
            maps.push(Map::new());
            continue;
        }
        maps.last_mut().unwrap().add(&line);
    }

    (seeds, maps)
}
