use super::*;

pub(super) fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file);
    0
}

fn parse_input(input_file: &str) -> () {
    iter_lines_from(input_file).map(|line| line);
}
