use super::*;
pub fn part1() {
    let limit = [12, 13, 14];
    let sum: u32 = iter_lines_from("res/2023/input02.txt")
        .map(|l| {
            //Game 5: 1 red, 3 blue, 15 green; 13 green, 2 blue; 6 green; 6 green, 8 blue; 4 green, 9 blue, 1 red
            l.split_once(": ")
                .expect("input error")
                .1
                .split("; ")
                .map(|set_line| {
                    set_line.split(", ").fold([0, 0, 0], |[r, g, b], part| {
                        match part.split_once(' ') {
                            Some((num, "red")) => [num.parse::<usize>().unwrap(), g, b],
                            Some((num, "green")) => [r, num.parse::<usize>().unwrap(), b],
                            Some((num, "blue")) => [r, g, num.parse::<usize>().unwrap()],
                            _ => unreachable!("Part"),
                        }
                    })
                })
                .collect_vec()
        })
        .enumerate()
        .filter(|(_, game)| {
            game.iter()
                .all(|set| set.iter().zip(limit.iter()).all(|(s, l)| s <= l))
        })
        .map(|(idx, _)| idx as u32 + 1)
        .sum();
    println!("{:?}", sum);
}

pub fn part2() {
    let sum: usize = iter_lines_from("res/2023/input02.txt")
        .map(|l| {
            //Game 5: 1 red, 3 blue, 15 green; 13 green, 2 blue; 6 green; 6 green, 8 blue; 4 green, 9 blue, 1 red
            l.split_once(": ")
                .expect("input error")
                .1
                .split("; ")
                .map(|set_line| {
                    set_line.split(", ").fold([0, 0, 0], |[r, g, b], part| {
                        match part.split_once(' ') {
                            Some((num, "red")) => [num.parse::<usize>().unwrap(), g, b],
                            Some((num, "green")) => [r, num.parse::<usize>().unwrap(), b],
                            Some((num, "blue")) => [r, g, num.parse::<usize>().unwrap()],
                            _ => unreachable!("Part"),
                        }
                    })
                })
                .collect_vec()
        })
        .map(|game| {
            game.iter()
                .fold([0, 0, 0], |set, part| {
                    [
                        set[0].max(part[0]),
                        set[1].max(part[1]),
                        set[2].max(part[2]),
                    ]
                })
                .iter()
                .product::<usize>()
        })
        .sum();
    println!("{:?}", sum);
}
