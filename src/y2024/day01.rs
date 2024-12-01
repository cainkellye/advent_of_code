use super::*;
pub fn part1() {
    //distances
    let (mut list1, mut list2) = read_lists();
    list1.sort();
    list2.sort();
    let sum: u32 = list1
        .into_iter()
        .zip(list2)
        .map(|(x, y)| x.abs_diff(y))
        .sum();
    println!("{:?}", sum);
}

pub fn part2() {
    let (list1, list2) = read_lists();
    let sum: usize = list1
        .into_iter()
        .map(|n| list2.iter().filter(|&&k| k == n).count() * n as usize)
        .sum();
    println!("{:?}", sum);
}

fn read_lists() -> (Vec<u32>, Vec<u32>) {
    let (list1, list2) = iter_lines_from("res/2024/input01.txt")
        .map(|l| {
            let (a, b) = l.split_once("   ").unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .fold((Vec::new(), Vec::new()), |(mut vec1, mut vec2), (a, b)| {
            vec1.push(a);
            vec2.push(b);
            (vec1, vec2)
        });
    (list1, list2)
}
