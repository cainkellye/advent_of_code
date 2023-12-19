use super::*;

pub fn part1() {
    println!("{:?}", part1::part1_internal("res/2023/input19.txt"));
}
pub fn part2() {
    println!("{:?}", part2::part2_internal("res/2023/input19.txt"));
}

mod part1;
mod part2;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2::part2_internal("res/2023/test19.txt"), 167409079868000);
    }
}
