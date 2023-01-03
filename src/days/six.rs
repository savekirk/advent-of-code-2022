use std::collections::{HashSet, VecDeque};

pub fn part_1(lines: Vec<String>) -> usize {
    char_before_marker::<4>(lines.first().unwrap())
}

pub fn part_2(lines: Vec<String>) -> usize {
    char_before_marker::<14>(lines.first().unwrap())
}

fn char_before_marker<const DC: usize>(line: &String) -> usize {
    let mut markers = line.chars().into_iter();
    let mut consumed = VecDeque::from(markers.next_chunk::<DC>().unwrap());
    let mut consumed_count = DC;

    while consumed.iter().clone().collect::<HashSet<&char>>().len() != DC {
        consumed.pop_front();
        consumed.push_back(markers.next().unwrap());
        consumed_count = consumed_count + 1;
    }

    consumed_count
}
