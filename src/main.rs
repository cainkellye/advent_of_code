use itertools::Itertools;
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

mod y2022;
mod y2023;

fn main() {
    let args = args().collect_vec();
    if args.get(1).unwrap_or(&String::new()) == "all" {
        y2023::solve_all();
        return;
    }
    let day: usize = args.get(1).map_or(1, |s| s.parse::<usize>().unwrap());
    let part: usize = args.get(2).map_or(1, |s| s.parse::<usize>().unwrap());
    y2023::solve(day, part);
}

fn iter_lines_from(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Can't open file.");
    let lines = BufReader::new(file);
    lines.lines().map_while(Result::ok)
}
