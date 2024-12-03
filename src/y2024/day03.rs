use super::*;
use regex::Regex;

pub fn part1() {
    println!("{:?}", part1_internal("res/2024/input03.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2024/input03.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    let re = Regex::new(r#"mul\(\d+,\d+\)"#).unwrap();
    iter_lines_from(input_file)
        .map(|line| {
            re.find_iter(&line)
                .map(|m| {
                    let (a, b) = m
                        .as_str()
                        .strip_prefix("mul(")
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .split_once(',')
                        .unwrap();
                    (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
                })
                .collect_vec()
        })
        .concat()
        .into_iter()
        .map(|(a, b)| a * b)
        .sum()
}

fn part2_internal(input_file: &str) -> usize {
    let re = Regex::new(r#"do\(\)|don't\(\)|mul\(\d+,\d+\)"#).unwrap();
    let lines = iter_lines_from(input_file).collect_vec();
    let matches = lines.iter().flat_map(|line| re.find_iter(line));
    let mut sum = 0;
    let mut enabled = true;
    for m in matches {
        match m.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            s => {
                if enabled {
                    let (a, b) = s
                        .strip_prefix("mul(")
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .split_once(',')
                        .unwrap();
                    sum += a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap();
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2024/input03.txt"), 166357705);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2024/input03.txt"), 88811886);
    }
}
