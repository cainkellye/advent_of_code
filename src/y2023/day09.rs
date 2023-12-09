use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input09.txt")); //1938731307
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input09.txt")); //948
}

fn part1_internal(input_file: &str) -> i32 {
    parse_input(input_file)
        .map(|mut list| extrapolate(list.as_mut_slice()))
        .sum()
}

fn part2_internal(input_file: &str) -> i32 {
    parse_input(input_file)
        .map(|mut list| {
            list.reverse();
            extrapolate(list.as_mut_slice())
        })
        .sum()
}

fn extrapolate(list: &mut [i32]) -> i32 {
    for k in (1..list.len()).rev() {
        //write the diff-s in place, left-aligned (leaving the last element intact)
        for n in 0..k {
            list[n] = list[n + 1] - list[n];
        }
        if list[0] == 0 && list[k - 1] == 0 {
            return list[k..].iter().sum();
        }
    }
    list.iter().sum()
}

fn parse_input(input_file: &str) -> impl Iterator<Item = Vec<i32>> {
    iter_lines_from(input_file).map(|line| {
        line.split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect_vec()
    })
}

/* Note my original reversible recursive extrapolate function:
    fn extrapolate(list: Vec<i32>, forward: bool) -> i32 {
        let idx = if forward { list.len() - 1 } else { 0 };
        let diffs = list.windows(2).map(|w| w[1] - w[0]).collect_vec();
        if diffs.iter().all(|&d| d == 0) {
            return list[idx];
        }
        list[idx] + extrapolate(diffs, forward)
    }

    Then I changed the diff calculation to be in-place instead of collecting
    into a new vector. I also modified the short-circuit check to only examine
    the first and last items.
    Then I focused on forward-case: the last item of the list is needed at the
    end when it is added to the result of the next recursion. Diffs are always 1
    less length, so I can store them left-aligned like: L[i] = L[i+1] - L[i])
    After this change I realized that the recursion just sums the items:
        line[len - 1] + extrapolate(line[0..len - 1])
    So change that to line.iter().sum() and done.
*/

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
