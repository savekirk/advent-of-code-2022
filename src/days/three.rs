use std::collections::HashSet;

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.

pub fn part_1(lines: Vec<String>) -> i32 {
    lines.iter().fold(0, |mut acc, line| {
        let len = line.chars().count();
        let c = line.split_at(len / 2);
        let c1 = c.0.chars().collect::<HashSet<char>>();
        let c2 = &c.1.chars().collect::<HashSet<char>>();
        acc = acc + priority(*c1.intersection(c2).last().unwrap()) as i32;
        acc
    })
}

pub fn part_2(lines: Vec<String>) -> i32 {
    lines.chunks(3).into_iter().fold(0, |mut acc, line| {
        let mut li = line.iter();
        let a = to_set(&mut li.next());
        let b = to_set(&mut li.next());
        let c = to_set(&mut li.next());
        let ab = a.intersection(&b).collect::<HashSet<&char>>();
        let ac = a.intersection(&c).collect::<HashSet<&char>>();
        let i = ab.intersection(&ac).last().unwrap();
        acc = acc + priority(**i) as i32;
        acc
    })
}

fn to_set(li: &mut Option<&String>) -> HashSet<char> {
    li.unwrap().chars().collect::<HashSet<char>>()
}

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        ((c as u32) % 65) + 27
    } else {
        ((c as u32) % 97) + 1
    }
}
