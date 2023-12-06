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
    fn translate(&self, value: usize) -> usize {
        let mapping = self
            .0
            .iter()
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
            let mapping = self
                .0
                .iter()
                .find(|&&(_, source, count)| source <= start && start < source + count);
            if let Some((dest, source, count)) = mapping {
                //if found, see how far we can go with this mapping
                let end = (start + count).min(source + count).min(range.end);
                //push the mapped range
                ranges.push(dest + start - source..dest + end - source);
                //continue from the end of this range
                start = end;
            } else if let Some((_, first, _)) = self
                .0
                .iter()
                .filter(|&&(_, source, _)| source > start)
                .min_by_key(|&&(_, source, _)| source)
            {
                //There were no mappings our start value fit
                //so we searched for the first mapping after our start
                //push a range unchanged up to the first value of that mapping
                ranges.push(start..*first);
                start = *first;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_translate_range_simple() {
        let mut map = Map::new();
        map.add("10 30 5");
        map.add("5 35 5");

        assert_eq!(map.translate_range(0..30), vec![(0..30)]);
        assert_eq!(map.translate_range(30..35), vec![(10..15)]);
        assert_eq!(map.translate_range(35..40), vec![(5..10)]);
        assert_eq!(map.translate_range(40..50), vec![(40..50)]);
    }

    #[test]
    fn test_translate_range_seed_to_soil() {
        let mut map = Map::new();
        map.add("50 98 2");
        map.add("52 50 48");

        assert_eq!(map.translate_range(79..93), vec![(81..95)]);
        assert_eq!(map.translate_range(55..68), vec![(57..70)]);
    }

    #[test]
    fn test_translate_range_soil_to_fertilizer() {
        let mut map = Map::new();
        map.add("0 15 37");
        map.add("37 52 2");
        map.add("39 0 15");

        assert_eq!(map.translate_range(57..70), vec![(57..70)]);
        assert_eq!(map.translate_range(81..95), vec![(81..95)]);
    }

    #[test]
    fn test_translate_range_multi() {
        let mut map = Map::new();
        map.add("10 30 5");
        map.add("5 35 5");

        assert_eq!(
            map.translate_range(20..40),
            vec![(20..30), (10..15), (5..10)]
        );
        assert_eq!(
            map.translate_range(30..50),
            vec![(10..15), (5..10), (40..50)]
        );
    }

    #[test]
    fn test_translate_range_shuffled() {
        let mut map = Map::new();
        map.add("60 5 10");
        map.add("5 35 5");
        map.add("40 20 5");
        map.add("10 30 5");

        assert_eq!(
            map.translate_range(20..45),
            vec![(40..45), (25..30), (10..15), (5..10), (40..45)]
        );
    }

    #[test]
    fn test_merge_ranges() {
        let ranges = vec![(40..45), (25..30), (10..15), (5..10), (40..45)];
        assert_eq!(merge_ranges(ranges), vec![(5..15), (25..30), (40..45)]);
    }
}
