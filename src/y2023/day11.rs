use super::*;
pub fn part1() {
    println!("{:?}", internal("res/2023/input11.txt", 2));
}
pub fn part2() {
    println!("{:?}", internal("res/2023/input11.txt", 1000000));
}

fn internal(input_file: &str, expansion: usize) -> usize {
    let space = parse_input(input_file);
    let (rows, cols) = (space.len(), space[0].len());
    let (expanded_rows, expanded_cols) = get_expanded_rows_cols(&space, rows, cols);
    let galaxies = space
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &col)| col == 1)
                .map(|(c, _)| (r, c))
                .collect_vec()
        })
        .collect_vec();
    galaxies
        .iter()
        .enumerate()
        .map(|(id, (r, c))| {
            galaxies
                .iter()
                .skip(id + 1)
                .map(|&(r2, c2)| {
                    let mut distance = r.abs_diff(r2) + c.abs_diff(c2);
                    distance += (r + 1..r2).filter(|r| expanded_rows.contains(r)).count() * (expansion - 1);
                    distance += (*c.min(&c2) + 1..*c.max(&c2))
                        .filter(|c| expanded_cols.contains(c))
                        .count() * (expansion - 1);
                    distance
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn get_expanded_rows_cols(
    space: &Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
) -> (Vec<usize>, Vec<usize>) {
    let mut expanded_rows = vec![];
    let mut expanded_cols = vec![];
    for row in 0..rows {
        if space[row].iter().all(|&c| c == 0) {
            expanded_rows.push(row);
        }
    }
    for col in 0..cols {
        if (0..rows).all(|r| space[r][col] == 0) {
            expanded_cols.push(col);
        }
    }
    (expanded_rows, expanded_cols)
}

fn parse_input(input_file: &str) -> Vec<Vec<u8>> {
    iter_lines_from(input_file)
        .map(|line| {
            line.bytes()
                .map(|b| if b == b'#' { 1 } else { 0 })
                .collect_vec()
        })
        .collect_vec()
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
