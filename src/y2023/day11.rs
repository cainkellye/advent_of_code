use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input11.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input11.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    parse_input(input_file);
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file);
}

fn parse_input(input_file: &str) -> () {
    iter_lines_from(input_file);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test11.txt"), ());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test11.txt"), ());
    }
}
