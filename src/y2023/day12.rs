use super::*;
use memoize::memoize;
use rayon::prelude::*;
use regex::Regex;

pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input12.txt")); // ? < 7676
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input12.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    let lines = iter_lines_from(input_file).collect_vec();
    lines.par_iter().map(|line| count_arrangements(line)).sum()
}

fn part2_internal(input_file: &str) -> usize {
    iter_lines_from(input_file)
        .par_bridge()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();
            let line = [springs; 5].join("?") + " " + &[groups; 5].join(",");
            count_arrangements(&line)
        })
        .sum()
}

fn count_arrangements(input: &str) -> usize {
    let (springs, groups) = input.split_once(' ').unwrap();
    let groups = groups
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();
    count_recursive(springs.to_owned(), groups)
}

#[memoize]
fn count_recursive(springs: String, groups: Vec<usize>) -> usize {
    let springs = springs.trim_start_matches('.');
    let mut count = 0;
    let pattern = format!(
        r"^[#\?]{{{}}}{}",
        groups[0],
        if groups.len() > 1 {
            r"[\.\?]"
        } else {
            r"[\.\?]*$"
        }
    );
    let re = Regex::new(&pattern).unwrap();
    if re.is_match(springs) {
        if groups.len() > 1 {
            if springs.len() > groups[0] + 1 {
                count +=
                    count_recursive(springs[groups[0] + 1..].to_owned(), groups[1..].to_owned());
            }
        } else {
            count += 1;
        }
    }
    let minimum_length = groups.iter().sum::<usize>() + groups.len() - 1;
    if springs.len() > minimum_length && springs.as_bytes()[0] == b'?' {
        count += count_recursive(springs[1..].to_owned(), groups);
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(count_arrangements("???.### 1,1,3"), 1);
        assert_eq!(count_arrangements(".??..??...?##. 1,1,3"), 4);
        assert_eq!(count_arrangements(".??..??...?##... 1,1,3"), 4);
        assert_eq!(count_arrangements("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(count_arrangements("?#?#?#?#?#?#?#?.. 1,3,1,6"), 1);
        assert_eq!(count_arrangements("?#?#?#?#?#?#?#?... 1,3,1,6"), 1);
        assert_eq!(count_arrangements("????.#...#... 4,1,1"), 1);
        assert_eq!(count_arrangements("????.######..#####. 1,6,5"), 4);
        assert_eq!(count_arrangements("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test12.txt"), 0);
    }
}
