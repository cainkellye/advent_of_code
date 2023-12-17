#![allow(unused)]
use itertools::Itertools;
use std::{
    fmt::{Debug, Formatter, Result, Write},
    ops::Range,
    time::SystemTime,
};

pub mod primes;

pub fn merge_ranges_in_place(mut ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    //sort the ranges by start
    ranges.sort_unstable_by_key(|r| r.start);
    let mut idx = 0;
    while idx < ranges.len() - 1 {
        //if the current range's end is after the next one's start
        if ranges[idx].end >= ranges[idx + 1].start {
            //merge them
            ranges[idx].end = ranges[idx + 1].end;
            ranges.remove(idx + 1);
        } else {
            //else check the next range
            idx += 1;
        }
    }
    ranges
}

#[cfg(not(debug_assertions))]
pub fn time(func: &fn()) {
    let start = SystemTime::now();
    func();
    let time = start.elapsed().unwrap_or_default();
    println!("### Time: {time:?}");
}

#[cfg(debug_assertions)]
pub fn time(func: &fn()) {
    func();
}

#[derive(Eq, PartialOrd, Ord, Clone)]
pub struct Grid {
    pub buffer: Vec<Vec<u8>>,
    pub rows: usize,
    pub cols: usize,
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.buffer == other.buffer
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('\n').unwrap();
        f.write_str(
            &self
                .buffer
                .iter()
                .map(|line| String::from_utf8_lossy(line))
                .join("\n"),
        )
    }
}

impl Grid {
    pub fn new(grid: Vec<Vec<u8>>) -> Self {
        Self {
            rows: grid.len(),
            cols: grid[0].len(),
            buffer: grid,
        }
    }
    pub fn row(&self, index: usize) -> &Vec<u8> {
        &self.buffer[index]
    }
    pub fn col(&self, index: usize) -> Vec<u8> {
        self.buffer.iter().map(|row| row[index]).collect_vec()
    }
    pub fn transposed(self) -> Self {
        Grid::new((0..self.cols).map(|col| self.col(col)).collect_vec())
    }
    pub fn item(&self, row: usize, col: usize) -> u8 {
        self.buffer[row][col]
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Part {
    One,
    Two,
}
