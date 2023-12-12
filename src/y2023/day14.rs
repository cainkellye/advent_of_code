use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input14.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input14.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    parse_input(input_file);
    0
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file);
    0
}

fn parse_input(input_file: &str) -> () {
    iter_lines_from(input_file).map(|line| line);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test14.txt"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test14.txt"), 0);
    }
}
