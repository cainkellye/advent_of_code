#![allow(unused)]
use super::iter_lines_from;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, BufRead};

fn day25() {
    let input = [
        "1=12=1--2220--2=21",
        "2=20---12",
        "1=2=-=2-0=0=022",
        "1212=--0-20=1",
        "1202-1",
        "2=2-=1-",
        "1-22-12-1",
        "1-1-1120=1=120221",
        "10-02",
        "122-12=",
        "2-2",
        "20001--=20100",
        "1-=-00=00=2",
        "1=--0200=--0=2--1-",
        "1==--001",
        "1-10011",
        "1-1=2-==2=2",
        "2-=02=02222-=0",
        "1-121=",
        "2012=2==",
        "1--",
        "1212-2",
        "1--10-112=",
        "100--1011-=-12---",
        "2=",
        "120",
        "1=12-1-==0==2-02",
        "1==10",
        "12==2201112-1=-",
        "22201=",
        "202-022=1111-0",
        "1-=1-=--2",
        "111011---",
        "2-=1122101--1-2-11",
        "22100",
        "1=20-112",
        "21=--1",
        "1-000--21=110101=",
        "2-10=02-121=",
        "112",
        "121=0-2000=-=01=12=",
        "11=-121-=-1",
        "1-20==---=000-",
        "111221==2-122111",
        "1=122=22010120121",
        "2-11=-22=2",
        "1-",
        "101=0121121-2122",
        "1=2221-11-2-=",
        "212",
        "11=-==00--",
        "121-0--1===111",
        "11",
        "2==00-21222=-2",
        "2=-02-=22-2=1",
        "22==-1222=-1-12--",
        "1--22",
        "2110-=2-0-211-12=",
        "202222-1111",
        "2-=-=1222221=",
        "1=-=-==-1=",
        "1-1-0121",
        "12-1==0222",
        "11---100",
        "1--01020-2",
        "10=20",
        "12=1=-002-02-=1222",
        "1011-0--0121-",
        "101-2==",
        "2112021212==212",
        "1-==1-=110-021-22",
        "1==20111-2022-0",
        "1=1=2-=0=",
        "1-0==11-1--",
        "1--21-21-0-2-",
        "11-111=-=10==10-",
        "12-",
        "202-10221=",
        "1-1=-00=0",
        "2-=11",
        "200-",
        "21=1=12022-01-0--=",
        "1=---0=021-0==",
        "1-=00-1-0210=2",
        "1121=1=2=20-",
        "220100-02111=2211-",
        "1-0-00-0200-001-",
        "1002",
        "12-=0-012-111",
        "1=-0111-10=0",
        "21=10-2=1=00-120-",
        "201=",
        "21",
        "2000=0-=",
        "11==0--212=--12-121",
        "11-220-0=22-011==",
        "1-1-0=1-21-101==",
        "111=",
        "2--1=2=-",
        "1==0=0-",
        "1=021222=0---2-1=",
        "2-20=210212-0122",
        "1-2100-021=-2=010-11",
        "1-=-1-0-1-=-0",
        "20-122200202-12-0=",
        "2=-20",
        "1-1-=-",
        "11=2=1=1-",
        "1=11-=-20=20",
    ];

    #[derive(Debug)]
    struct Snafu {
        val: i64,
        repr: String,
    }

    impl From<&str> for Snafu {
        fn from(value: &str) -> Self {
            let mut val = 0;
            for (i, digit) in value.chars().rev().enumerate() {
                val += 5_i64.pow(i.try_into().unwrap())
                    * match digit {
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        '-' => -1,
                        '=' => -2,
                        _ => unreachable!("x"),
                    };
            }
            Self {
                val,
                repr: value.into(),
            }
        }
    }

    impl From<i64> for Snafu {
        fn from(value: i64) -> Self {
            let mut val = value;
            let exp = (0_u32..29).find(|n| 5_i64.pow(*n) > val).unwrap() as usize;
            let mut repr = vec![0_i64; exp + 1];
            for exp in (0..exp).rev() {
                let k = 5_i64.pow(exp.try_into().unwrap());
                let x = val / k;
                val -= x * k;
                match repr[exp] + x {
                    0 => repr[exp] = 0,
                    1 => repr[exp] = 1,
                    2 => repr[exp] = 2,
                    3 => {
                        repr[exp] = -2;
                        add_one(&mut repr, exp + 1)
                    }
                    4 => {
                        repr[exp] = -1;
                        add_one(&mut repr, exp + 1)
                    }
                    _ => unreachable!("1"),
                }
            }
            if repr[repr.len() - 1] == 0 {
                repr.pop();
            }
            Self {
                val: value,
                repr: repr
                    .iter()
                    .rev()
                    .map(|n| match n {
                        0 => '0',
                        1 => '1',
                        2 => '2',
                        -1 => '-',
                        -2 => '=',
                        _ => unreachable!("2"),
                    })
                    .collect::<String>(),
            }
        }
    }

    fn add_one(repr: &mut Vec<i64>, exp: usize) {
        repr[exp] += 1;
        if repr[exp] == 3 {
            repr[exp] = -2;
            add_one(repr, exp + 1);
        }
    }

    let sum = input.into_iter().map(|a| Snafu::from(a).val).sum::<i64>();
    //> 36966761092496
    let result = Snafu::from(sum); //Snafu::from("20=212=1-12=200=00-1");
    println!("{result:?}");
    //> Snafu { val: 36966761092496, repr: "20=212=1-12=200=00-1" }
}

fn day11() {
    #[derive(Debug)]
    enum Operation<'a> {
        Add(&'a str),
        Mul(&'a str),
    }

    impl<'a> Operation<'a> {
        fn execute(&self, item: &u128) -> u128 {
            match self {
                Operation::Add(s) => {
                    if *s == "old" {
                        item + item
                    } else {
                        item + s.parse::<u128>().unwrap()
                    }
                }
                Operation::Mul(s) => {
                    if *s == "old" {
                        item * item
                    } else {
                        item * s.parse::<u128>().unwrap()
                    }
                }
            }
        }
    }

    #[derive(Debug)]
    struct Monkey<'a> {
        items: Vec<u128>,
        operation: Operation<'a>,
        test_mod: u128,
        test_ok: u32,
        test_nok: u32,
        inspections: u128,
    }

    let input = std::fs::read_to_string("res/input11.txt").unwrap();
    let mut monkeys = Vec::<Monkey>::new();
    for monkey_init in input.split("\n\n") {
        let mut monkey = Monkey {
            items: Vec::new(),
            operation: Operation::Add(""),
            test_mod: 0,
            test_ok: 0,
            test_nok: 0,
            inspections: 0,
        };
        for params in monkey_init.split('\n') {
            match params.trim().split_once(':') {
                Some(("Starting items", i)) => monkey
                    .items
                    .extend(i.split(", ").map(|i| i.trim().parse::<u128>().unwrap())),
                Some(("Operation", op)) => match op
                    .strip_prefix(" new = old ")
                    .unwrap_or_else(|| panic!("new = old cannot be stripped, {op:?}"))
                    .rsplit_once(' ')
                {
                    Some(("+", op)) => monkey.operation = Operation::Add(op),
                    Some(("*", op)) => monkey.operation = Operation::Mul(op),
                    _ => panic!("Missing Operation: {op}"),
                },
                Some(("Test", test)) => {
                    monkey.test_mod = test
                        .strip_prefix(" divisible by ")
                        .unwrap_or_else(|| panic!("divisible by cannot be stripped, {test:?}"))
                        .parse()
                        .unwrap()
                }
                Some(("If true", test_ok)) => {
                    monkey.test_ok = test_ok.rsplit_once(' ').unwrap().1.parse().unwrap()
                }
                Some(("If false", test_nok)) => {
                    monkey.test_nok = test_nok.rsplit_once(' ').unwrap().1.parse().unwrap()
                }
                Some((monkey_id, "")) => {
                    monkey_id
                        .split_once(' ')
                        .expect("Monkey ID split is wrong.");
                }
                None => (),
                _ => unreachable!(),
            };
        }
        monkeys.push(monkey);
    }
    println!("{:?}", monkeys);
    let cap = monkeys.iter().map(|m| m.test_mod).product::<u128>();
    let rounds = 10000;
    for _ in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            //Monkey inspects his items
            while let Some(mut item) = monkeys[monkey_idx].items.pop() {
                item = monkeys[monkey_idx].operation.execute(&item);
                monkeys[monkey_idx].inspections += 1;
                // gets bored
                // item /= 3;
                item %= cap;
                let to_monkey = (if item % monkeys[monkey_idx].test_mod == 0 {
                    monkeys[monkey_idx].test_ok
                } else {
                    monkeys[monkey_idx].test_nok
                }) as usize;
                monkeys[to_monkey].items.push(item);
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspections);
    monkeys.reverse();
    let monkey_business: u128 = monkeys.iter().take(2).map(|m| m.inspections).product();
    println!("Monkey business after 20 rounds: {monkey_business}");
}

fn day10_part2() {
    let mut x = 1;
    let mut beam = 0;
    let mut screen = String::new();
    for line in iter_lines_from("res/input10.txt") {
        process_cycle(&mut beam, &x, &mut screen);
        if let Some((_, arg)) = line.split_once(' ') {
            process_cycle(&mut beam, &x, &mut screen);
            x += arg.parse::<i32>().unwrap();
        }
    }
    fn process_cycle(beam: &mut i32, x: &i32, screen: &mut String) {
        screen.push(if beam.abs_diff(*x) < 2 { '▓' } else { ' ' });
        *beam = if *beam < 39 {
            *beam + 1
        } else {
            screen.push('\n');
            0
        };
    }
    println!("{screen}");
}

fn day10() {
    let mut x = 1i64;
    let mut cycle = 1;
    let mut sum = 0i64;
    fn process_cycle(times: u8, cycle: &mut i64, sum: &mut i64, x: &i64) {
        for _ in 0..times {
            if [20, 60, 100, 140, 180, 220].contains(cycle) {
                *sum += *x * *cycle;
            }
            *cycle += 1;
        }
    }
    for line in iter_lines_from("res/input10.txt") {
        if let Some((command, arg)) = line.split_once(' ') {
            if command != "addx" {
                panic!("Not addx ???");
            }
            let arg: i64 = arg.parse().unwrap();
            process_cycle(2, &mut cycle, &mut sum, &x);
            x += arg;
        } else {
            if line != "noop" {
                panic!("Not noop ???");
            }
            process_cycle(1, &mut cycle, &mut sum, &x);
        }
    }
    println!("{:?}", sum);
}

fn day09() {
    const SIZE: usize = 10; //Part1 : 2;
    fn catch_up(rope: &mut [(i16, i16); SIZE], pos: usize) {
        if rope[pos - 1].0.abs_diff(rope[pos].0) < 2 && rope[pos - 1].1.abs_diff(rope[pos].1) < 2 {
            return;
        }
        rope[pos].1 += (rope[pos - 1].1 - rope[pos].1).signum();
        rope[pos].0 += (rope[pos - 1].0 - rope[pos].0).signum();
    }
    let mut visited = HashSet::new();
    let mut rope = [(0i16, 0i16); SIZE];
    for line in iter_lines_from("res/input09.txt") {
        let (direction, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse().unwrap();
        for _ in 0..steps {
            match direction {
                "R" => rope[0].1 += 1,
                "L" => rope[0].1 -= 1,
                "D" => rope[0].0 += 1,
                "U" => rope[0].0 -= 1,
                _ => unreachable!(),
            }
            for i in 1..rope.len() {
                catch_up(&mut rope, i);
            }
            // println!("{:?} {:?} {:?} {:?}", direction, steps, head, tail);
            visited.insert(rope[SIZE - 1]);
        }
    }
    println!("{:?}", visited.len());
}

fn day08() {
    use ndarray::prelude::*;
    let size = 99;
    let mut a = Array::zeros((0, size));
    for line in iter_lines_from("res/input08.txt") {
        a.push_row(ArrayView::from(
            &line.as_bytes().iter().map(|x| (x - 48) as i8).collect_vec(),
        ))
        .unwrap();
    }
    // Part 1
    let mut visible = Array::<bool, Ix2>::from_elem((size, size).f(), false);
    for i in 0..size {
        let row = a.slice(s![i, ..]);
        let col = a.slice(s![.., i]);
        let (mut left_max, mut up_max) = (-1, -1);
        let (mut right_max, mut down_max) = (-1, -1);
        for j in 0..size {
            if row[j] > left_max {
                visible[[i, j]] = true;
                left_max = row[j];
            }
            if col[j] > up_max {
                visible[[j, i]] = true;
                up_max = col[j];
            }
            if row[size - j - 1] > right_max {
                visible[[i, size - j - 1]] = true;
                right_max = row[size - j - 1];
            }
            if col[size - j - 1] > down_max {
                visible[[size - j - 1, i]] = true;
                down_max = col[size - j - 1];
            }
        }
    }
    // Part 2
    let mut max_score = 0;
    for i in 0..size {
        for j in 0..size {
            /*          i-
            j- <- (i, j) -> j+
                    i+          */
            let pos = [i, j];
            let up = count(a.slice(s![..i;-1, j]).into_iter(), &a, &pos);
            let down = count(a.slice(s![i.., j]).into_iter().skip(1), &a, &pos);
            let left = count(a.slice(s![i, ..j;-1]).into_iter(), &a, &pos);
            let right = count(a.slice(s![i, j..]).into_iter().skip(1), &a, &pos);

            let score = up * down * left * right;
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!(
        "Visible trees: {}",
        visible.into_iter().filter(|&n| n).count()
    );
    println!("Max scenic score: {}", max_score);
}

fn count<'a>(
    up: impl Iterator<Item = &'a i8>,
    a: &ndarray::ArrayBase<ndarray::OwnedRepr<i8>, ndarray::Dim<[usize; 2]>>,
    pos: &[usize; 2],
) -> usize {
    let mut count = 0;
    for n in up {
        match n.cmp(&a[*pos]) {
            std::cmp::Ordering::Less => count += 1,
            std::cmp::Ordering::Equal => {
                count += 1;
                break;
            }
            std::cmp::Ordering::Greater => break,
        }
    }
    count
}

fn day07() {
    let cd = Regex::new(r"\$ cd (?P<dir>.+)").unwrap();
    let file = Regex::new(r"(?P<size>\d+) (?P<file_name>.*)").unwrap();
    let mut file_table = HashMap::new();
    let mut path = String::new();
    for line in iter_lines_from("res/input07.txt") {
        if let Some(captures) = cd.captures(&line) {
            match &captures["dir"] {
                "/" => path = String::from("/"),
                ".." => {
                    if let Some((left_part, _)) = path.rsplit_once('/') {
                        path = left_part.to_owned();
                    }
                }
                s => path = format!("{path}/{s}"),
            }
        }
        if let Some(captures) = file.captures(&line) {
            let size = captures["size"].parse::<u32>().unwrap();
            file_table
                .entry(path.clone())
                .and_modify(|val| *val += size)
                .or_insert(size);
            let mut upper = path.clone();
            while let Some((_upper, _)) = upper.rsplit_once('/') {
                upper = _upper.to_owned();
                file_table
                    .entry(upper.clone())
                    .and_modify(|val| *val += size)
                    .or_insert(size);
            }
        }
    }
    let unused = 70_000_000 - file_table["/"];
    let min_to_delete = 30_000_000 - unused;
    println!(
        "{:?}",
        file_table
            .into_iter()
            .filter_map(|x| if x.1 >= min_to_delete {
                Some(x.1)
            } else {
                None
            })
            .min()
            .unwrap()
    );
}

fn day06() {
    let input = std::fs::read_to_string("res/input06.txt").unwrap();
    for i in 4..input.len() {
        if HashSet::<char>::from_iter(input[i - 4..i].chars()).len() == 4 {
            println!("Part1: {}", i);
            break;
        }
    }
    for i in 14..input.len() {
        if HashSet::<char>::from_iter(input[i - 14..i].chars()).len() == 14 {
            println!("Part2: {}", i);
            break;
        }
    }
}

fn day05() {
    let mut lines = iter_lines_from("res/input05.txt");
    let mut stack_lines: Vec<String> = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        } else {
            stack_lines.push(line);
        }
    }
    let stacks_count = stack_lines
        .pop()
        .unwrap()
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;
    let mut stacks: Vec<Vec<String>> = vec![Vec::new(); stacks_count];
    while let Some(x) = stack_lines.pop() {
        for i in 0..9 {
            let c = x[i * 4..i * 4 + 3].to_string();
            if c.trim().is_empty() {
                continue;
            }
            stacks[i].push(c);
        }
    }
    // println!("{stacks:?}");
    let move_pattern =
        Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    for line in lines {
        let values = move_pattern.captures(line.as_str()).unwrap();
        let count = values["count"].parse::<usize>().unwrap();
        let from = values["from"].parse::<usize>().unwrap() - 1;
        let to = values["to"].parse::<usize>().unwrap() - 1;
        // Part1
        for _ in 0..count {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
        // Part2
        // let from_len = stacks[from].len();
        // let mut moves = stacks[from].drain(from_len-count..).collect_vec();
        // stacks[to].append(&mut moves);
    }
    let mut tops = String::new();
    for item in stacks.iter() {
        tops += &item.last().unwrap()[1..2];
    }
    // println!("{stacks:?}");
    println!("{tops:?}");
}

fn day04() {
    let mut res_part1 = 0;
    let mut res_part2 = 0;
    for line in iter_lines_from("res/input04.txt") {
        struct Range {
            bottom: u32,
            top: u32,
        }
        let (left, right) = line
            .split(',')
            .map(|s| {
                let bounds = s
                    .split('-')
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect_vec();
                Range {
                    bottom: bounds[0],
                    top: bounds[1],
                }
            })
            .collect_tuple()
            .unwrap();
        if left.bottom <= right.bottom && left.top >= right.top
            || left.bottom >= right.bottom && left.top <= right.top
        {
            res_part1 += 1;
        }
        if left.bottom <= right.top && left.top >= right.bottom {
            res_part2 += 1;
        }
    }
    println!("{res_part1}, {res_part2}");
}

fn day03_part2() {
    let lines = iter_lines_from("res/input03.txt");
    let res = &lines
        .chunks(3)
        .into_iter()
        .map(|mut c| {
            let first = c.next().unwrap().chars().collect::<HashSet<_>>();
            let same = *c
                .fold(first, |a: HashSet<_>, l: String| {
                    (&a & &l.chars().collect::<HashSet<_>>())
                        .into_iter()
                        .collect()
                })
                .iter()
                .next()
                .unwrap() as u32;
            if same < 97 {
                same - 38
            } else {
                same - 96
            }
        })
        .sum::<u32>();
    println!("{res}");
}

fn day03() {
    let lines = iter_lines_from("res/input03.txt");
    let res = lines
        .map(|l| {
            let set = *l[..l.len() / 2]
                .chars()
                .collect::<HashSet<_>>()
                .intersection(&l[l.len() / 2..].chars().collect::<HashSet<_>>())
                .next()
                .unwrap() as u32;
            if set < 97 {
                set - 38
            } else {
                set - 96
            }
        })
        .sum::<u32>();
    println!("{res}");
}

fn day02() {
    let lines = iter_lines_from("res/input02.txt");
    // Rock Paper Scissors
    let mut score = 0;
    for line in lines {
        let turn = line.split(' ').collect_vec();
        let turn = (
            match turn[0] {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => unreachable!(),
            },
            match turn[1] {
                "X" => match turn[0] {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => unreachable!(),
                },
                "Y" => match turn[0] {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    _ => unreachable!(),
                },
                "Z" => match turn[0] {
                    "A" => 2,
                    "B" => 3,
                    "C" => 1,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
        );
        score += turn.1;
        score += match turn.0 {
            1 => match turn.1 {
                1 => 3,
                2 => 6,
                3 => 0,
                _ => unreachable!(),
            },
            2 => match turn.1 {
                2 => 3,
                3 => 6,
                1 => 0,
                _ => unreachable!(),
            },
            3 => match turn.1 {
                3 => 3,
                1 => 6,
                2 => 0,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    println!("{score}");
}

fn day01() {
    let lines = iter_lines_from("res/input01.txt");
    let max = lines
        .map(|l| l.parse::<i32>())
        .coalesce(|x, y| match (&x, &y) {
            (&Ok(x), &Ok(y)) => Ok(Ok(x + y)),
            _ => Err((x, y)),
        })
        .filter_map(|x| x.ok())
        .sorted()
        .rev()
        .take(3)
        .sum::<i32>();

    println!("{:?}", max);
}
