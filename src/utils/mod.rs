#![allow(unused)]
use std::{ops::Range, time::SystemTime};
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