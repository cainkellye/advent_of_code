use super::*;
pub fn part1() {
    println!("{:?}", internal("res/2023/input11.txt", 2));
}
pub fn part2() {
    println!("{:?}", internal("res/2023/input11.txt", 1000000));
}

fn internal(input_file: &str, expansion: usize) -> usize {
    let (space, galaxies) = parse_input(input_file);
    let (empty_rows, empty_cols) = get_empty_rows_cols(&space);
    galaxies
        .iter()
        .enumerate()
        .map(|(id, (r, c))| {
            galaxies
                .iter()
                .skip(id + 1)
                .map(|&(r2, c2)| {
                    let mut distance = r.abs_diff(r2) + c.abs_diff(c2);
                    distance += (r + 1..r2) // here we know that r2 will always be the bigger
                        .filter(|r| empty_rows.contains(r))
                        .count()
                        * (expansion - 1);
                    distance += (*c.min(&c2) + 1..*c.max(&c2)) // here we don't know
                        .filter(|c| empty_cols.contains(c))
                        .count()
                        * (expansion - 1);
                    distance
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn get_empty_rows_cols(space: &Vec<Vec<u8>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];
    let (rows, cols) = (space.len(), space[0].len());
    (0..rows)
        .filter(|&row| space[row].iter().all(|&c| c == 0))
        .for_each(|row| empty_rows.push(row));
    (0..cols)
        .filter(|&col| (0..rows).all(|r| space[r][col] == 0))
        .for_each(|col| empty_cols.push(col));
    (empty_rows, empty_cols)
}

fn parse_input(input_file: &str) -> (Vec<Vec<u8>>, Vec<(usize, usize)>) {
    let mut galaxies = vec![];
    let space = iter_lines_from(input_file)
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, b)| {
                    if b == b'#' {
                        galaxies.push((row, col));
                        1
                    } else {
                        0
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    (space, galaxies)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(internal("res/2023/test11.txt", 2), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(internal("res/2023/test11.txt", 10), 1030);
        assert_eq!(internal("res/2023/test11.txt", 100), 8410);
    }
}
