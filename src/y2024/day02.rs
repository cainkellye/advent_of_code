use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2024/input02.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2024/input02.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    parse_input(input_file)
        .filter(|list| verify(list.as_slice()))
        .count()
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file)
        .filter(|list| {
            for n in 0..list.len() {
                let one_left_out = [&list[..n], &list[n + 1..]].concat();
                if verify(&one_left_out) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn verify(list: &[i32]) -> bool {
    let sign = (list[0] - list[1]).signum();
    list.windows(2).all(|pair| {
        let diff = pair[0] - pair[1];
        diff.abs() <= 3 && diff.signum() == sign && sign != 0
    })
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
        assert_eq!(part1_internal("res/2024/input02.txt"), 299);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2024/input02.txt"), 364);
    }
}
