use super::*;

#[test]
fn test_part1() {
    let grid1 = Grid::new(vec![
        b"#.##..##.".to_vec(),
        b"..#.##.#.".to_vec(),
        b"##......#".to_vec(),
        b"##......#".to_vec(),
        b"..#.##.#.".to_vec(),
        b"..##..##.".to_vec(),
        b"#.#.##.#.".to_vec(),
    ]);

    let grid2 = Grid::new(vec![
        b"#...##..#".to_vec(),
        b"#....#..#".to_vec(),
        b"..##..###".to_vec(),
        b"#####.##.".to_vec(),
        b"#####.##.".to_vec(),
        b"..##..###".to_vec(),
        b"#....#..#".to_vec(),
    ]);

    let grid3 = Grid::new(vec![
        b".......##..##".to_vec(),
        b"..##...#.#...".to_vec(),
        b".#..#...#.###".to_vec(),
        b".####...#..##".to_vec(),
        b"##..##.####.#".to_vec(),
        b"#.##.#.#.....".to_vec(),
        b".......#.##..".to_vec(),
        b"..##..#.#.###".to_vec(),
        b"......#..#..#".to_vec(),
        b".......#.##.#".to_vec(),
        b"......#..#.##".to_vec(),
        b"#...##..#..##".to_vec(),
        b".####.#.#..##".to_vec(),
        b".####.#.##...".to_vec(),
        b".####.#.##...".to_vec(),
    ]);

    let grid4 = Grid::new(vec![
        b"...##.#.#..##".to_vec(),
        b".#.#..#.#..##".to_vec(),
        b".##.##...#..#".to_vec(),
        b".##.##...#..#".to_vec(),
        b".#.#..#.#..##".to_vec(),
        b"...##.#.#..##".to_vec(),
        b".#.#.##....#.".to_vec(),
        b"########....#".to_vec(),
        b"##..#.#...#.#".to_vec(),
        b"......##.....".to_vec(),
        b"#..#.....###.".to_vec(),
        b"#.#.#####.##.".to_vec(),
        b"#.#.#####.##.".to_vec(),
        b"#..#.....###.".to_vec(),
        b"......##.....".to_vec(),
        b"##..#.#.#.#.#".to_vec(),
        b"########....#".to_vec(),
    ]);

    assert_eq!(part1::find_mirror(&grid1), 5);
    assert_eq!(part1::find_mirror(&grid2), 400);
    assert_eq!(part1::find_mirror(&grid3), 1400);
    assert_eq!(part1::find_mirror(&grid4), 300);
}

#[test]
fn test_part2() {
    let grid1 = Grid::new(vec![
        b"#.##..##.".to_vec(),
        b"..#.##.#.".to_vec(),
        b"##......#".to_vec(),
        b"##......#".to_vec(),
        b"..#.##.#.".to_vec(),
        b"..##..##.".to_vec(),
        b"#.#.##.#.".to_vec(),
    ]);

    let grid2 = Grid::new(vec![
        b"#...##..#".to_vec(),
        b"#....#..#".to_vec(),
        b"..##..###".to_vec(),
        b"#####.##.".to_vec(),
        b"#####.##.".to_vec(),
        b"..##..###".to_vec(),
        b"#....#..#".to_vec(),
    ]);

    assert_eq!(part2::find_mirror(&grid1), 300);
    assert_eq!(part2::find_mirror(&grid2), 100);
}
