use itertools::Itertools;
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader, Read},
};

mod y2022;
mod y2023;
mod utils;

fn main() {
    let args = args().collect_vec();
    if args.get(1).unwrap_or(&String::new()) == "all" {
        y2023::solve_all();
        return;
    }
    let day: usize = args.get(1).map_or(1, |s| s.parse::<usize>().unwrap());
    if args.len() == 2 {
        y2023::solve(day, 1);
        y2023::solve(day, 2);
        return;
    }
    let part: usize = args.get(2).map_or(1, |s| s.parse::<usize>().unwrap());
    y2023::solve(day, part);
}

fn iter_lines_from(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Can't open file.");
    let lines = BufReader::new(file);
    lines.lines().map_while(Result::ok)
}

#[allow(unused)]
fn read_to_string(path: &str) -> String {
    let mut file = File::open(path).expect("Can't open file.");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Can't read file.");
    input
}