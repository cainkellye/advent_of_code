use super::*;
use crate::read_to_string;

pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input15.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input15.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    read_to_string(input_file).split(',').map(hash).sum()
}

fn hash(text: &str) -> usize {
    text.chars()
        .fold(0, |hash, ch| ((hash + (ch as usize)) * 17) % 256)
}

fn part2_internal(input_file: &str) -> usize {
    let mut boxes: [Vec<(String, usize)>; 256] = std::array::from_fn(|_| Vec::new());
    for part in read_to_string(input_file).split(',') {
        if let Some((label, focal_length)) = part.split_once('=') {
            let _box = &mut boxes[hash(label)];
            let label = label.to_owned();
            let focal_length = focal_length.parse::<usize>().unwrap();
            if let Some(idx) = _box.iter().position(|(l, _)| l == &label) {
                _box[idx].1 = focal_length;
            } else {
                _box.push((label, focal_length));
            }
        } else {
            let label = &part[..part.len() - 1];
            let _box = &mut boxes[hash(label)];
            if let Some(idx) = _box.iter().position(|(l, _)| l == label) {
                _box.remove(idx);
            } else {
                // nothing
            }
        }
    }
    boxes
        .into_iter()
        .zip(1..)
        .flat_map(|(_box, box_num)| {
            _box.into_iter()
                .zip(1..)
                .map(|((_, focal_length), slot)| box_num * slot * focal_length)
                .collect_vec()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test15.txt"), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test15.txt"), 145);
    }
}
