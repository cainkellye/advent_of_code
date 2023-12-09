use super::*;

pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input10.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input10.txt"));
}

fn part1_internal(input_file: &str) {}

fn part2_internal(input_file: &str) {}

fn parse_input(input_file: &str) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test10.txt"), ());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test10.txt"), ());
    }
}
