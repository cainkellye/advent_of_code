use super::*;
use std::ops::Range;

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
    let ranges = seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect_vec();
    // Run each map on the ranges
    let translated_ranges = maps.iter().fold(ranges, |ranges, map| {
        merge_ranges(
            //Send every range into the current map to translate the range
            ranges
                .into_iter()
                .flat_map(|range| map.translate_range(range))
                .collect_vec(),
        )
    });
    let min = translated_ranges.iter().map(|r| r.start).min().unwrap();
    println!("{min}");
}

#[derive(Debug)]
/// dest, source, count
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
    fn iter_mappings(&self) -> impl Iterator<Item = &(usize, usize, usize)> {
        self.0.iter()
    }
    fn translate(&self, value: usize) -> usize {
        let mapping = self
            .iter_mappings()
            .find(|&&(_, source, count)| source <= value && value < source + count);
        if let Some((dest, source, _)) = mapping {
            dest + value - source
        } else {
            value
        }
    }
    fn translate_range(&self, range: Range<usize>) -> Vec<Range<usize>> {
        let mut start = range.start;
        let mut ranges = vec![];
        while start < range.end {
            //find the mapping that the current start value fit in
            if let Some((dest, source, count)) = self
                .iter_mappings()
                .find(|&&(_, source, count)| source <= start && start < source + count)
            {
                //if found, see how far we can go with this mapping
                let end = (start + count).min(source + count).min(range.end);
                //push the mapped range
                ranges.push(dest + start - source..dest + end - source);
                //continue from the end of this range
                start = end;
            } else if let Some((_, source, _)) = self
                .iter_mappings()
                .filter(|&&(_, source, _)| source > start)
                .min_by_key(|&&(_, source, _)| source)
            {
                //There were no mappings our start value fit
                //so we searched for the first mapping after our start
                //push an unmapped range up to the first value of that mapping
                ranges.push(start..*source);
                start = *source;
            } else {
                //We are past the end of every mapping
                //Push the rest of the range unchanged
                ranges.push(start..range.end);
                start = range.end;
            }
        }
        ranges
    }
}

fn parse_input() -> (Vec<usize>, Vec<Map>) {
    let mut lines = iter_lines_from("res/2023/input05.txt");
    let first_line = lines.next().unwrap();
    let (_, seeds) = first_line.split_once(": ").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();

    let mut maps = Vec::<Map>::new();
    for line in lines.filter(|l| !l.is_empty()) {
        if line.chars().next().unwrap().is_alphabetic() {
            maps.push(Map::new());
            continue;
        }
        maps.last_mut().unwrap().add(&line);
    }
    (seeds, maps)
}

fn merge_ranges(mut ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    //first, sort the ranges
    ranges.sort_unstable_by_key(|r| r.start);
    ranges.into_iter().coalesce(|current, next|
        //if the current range's end is after the next one's start
        if current.end >= next.start {
            Ok(current.start..next.end) //merge them
        } else {
            Err((current, next)) //else leave them
        }).collect_vec()
}

#[cfg(test)]
mod test;
