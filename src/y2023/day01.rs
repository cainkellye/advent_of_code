use super::*;
pub fn part1() {
    let sum: u32 = iter_lines_from("res/2023/input01.txt")
        .map(|l| {
            let digits = l.chars().filter_map(|c| c.to_digit(10)).collect_vec();
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum();
    println!("{:?}", sum);
}

pub fn part2() {
    let digits: Vec<(String, _)> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .flat_map(|(i, &digit)| [(digit.into(), i + 1), (format!("{}", i + 1), i + 1)])
    .collect();

    let sum: usize = iter_lines_from("res/2023/input01.txt")
        .map(|l| {
            let (_, first) = digits
                .iter()
                .min_by_key(|(s, _)| l.find(s.as_str()).unwrap_or(usize::MAX))
                .unwrap();
            let (_, last) = digits
                .iter()
                .max_by_key(|(s, _)| l.rfind(s.as_str()).map_or(-1, |x| x as i32))
                .unwrap();
            first * 10 + last
        })
        .sum();
    println!("{:?}", sum);
}
