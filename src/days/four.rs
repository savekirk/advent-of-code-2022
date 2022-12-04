use std::collections::HashSet;

pub fn part_1(lines: Vec<String>) -> i32 {
    lines.iter().fold(0, |mut acc, line| {
        let (s1, s2) = line
            .split_once(',')
            .and_then(|(s1, s2)| Some((to_set(s1), to_set(s2))))
            .unwrap();
        if s1.is_subset(&s2) || s2.is_subset(&s1) {
            acc = acc + 1
        }
        acc
    })
}

pub fn part_2(lines: Vec<String>) -> i32 {
    lines.iter().fold(0, |mut acc, line| {
        let (s1, s2) = line
            .split_once(',')
            .and_then(|(s1, s2)| Some((to_set(s1), to_set(s2))))
            .unwrap();
        if !s1.is_disjoint(&s2) {
            acc = acc + 1
        }
        acc
    })
}

fn to_set(section: &str) -> HashSet<u8> {
    section
        .split_once('-')
        .and_then(|(l, u)| {
            Some((l.parse::<u8>().unwrap()..=u.parse::<u8>().unwrap()).collect::<HashSet<u8>>())
        })
        .unwrap()
}
