use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input09.txt")); //1938731307
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input09.txt")); //948
}

fn part1_internal(input_file: &str) -> i32 {
    let input = parse_input(input_file);
    input.into_iter().map(|list| extrapolate(list, true)).sum()
}

fn part2_internal(input_file: &str) -> i32 {
    let input = parse_input(input_file);
    input.into_iter().map(|list| extrapolate(list, false)).sum()
}

fn extrapolate(list: Vec<i32>, forward: bool) -> i32 {
    let distances = list.windows(2).map(|w| w[1] - w[0]).collect_vec();
    if distances.iter().all(|&d| d == 0) {
        return list[if forward { list.len() - 1 } else { 0 }];
    }
    list[if forward { list.len() - 1 } else { 0 }] + extrapolate(distances, forward)
}

fn parse_input(input_file: &str) -> impl Iterator<Item = Vec<i32>> {
    iter_lines_from(input_file).map(|line| {
        line.split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect_vec()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test09.txt"), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test09.txt"), 2);
    }
}
