use std::collections::{HashMap, LinkedList};

pub fn part_1(lines: Vec<String>) -> String {
    move_creates(lines, false)
}

fn move_creates(lines: Vec<String>, can_move_multiple: bool) -> String {
    let mut stacks: HashMap<usize, LinkedList<char>> = HashMap::new();
    let ts = (&lines.first().unwrap().len() / 4) + 1;
    let mut parse_commands = false;
    for line in &lines {
        if line.trim().is_empty() {
            parse_commands = true;
            continue;
        }
        if parse_commands {
            let command = &mut line.trim().split(" ");
            let mut m = Vec::from([1, 1, 1])
                .into_iter()
                .map(|_| command.nth(1).unwrap().parse::<usize>().unwrap())
                .into_iter();
            let to_move = m.next().unwrap();
            let origin = m.next().unwrap();
            let dest = m.next().unwrap();
            let mut new_list = LinkedList::new();
            for _ in 1..=to_move {
                let v = stacks.get(&origin).unwrap().to_owned().pop_back().unwrap();
                stacks.entry(origin).and_modify(|s| {
                    s.pop_back();
                });
                if !can_move_multiple {
                    stacks.entry(dest).and_modify(|s| s.push_back(v));
                } else {
                    new_list.push_front(v);
                }
            }

            if can_move_multiple {
                stacks.entry(dest).and_modify(|s| s.append(&mut new_list));
            }
        } else {
            for si in 0..ts {
                let to_pick = if si == ts - 1 { 3 } else { 4 };
                let v = &line[(si * 4)..((si * 4) + to_pick)].trim().chars().nth(1);
                if let Some(c) = v {
                    stacks
                        .entry(si + 1)
                        .and_modify(|s| s.push_front(*c))
                        .or_insert(LinkedList::from([*c]));
                }
            }
        }
    }
    let mut stacks = stacks.into_iter().fold(Vec::new(), |mut acc, (i, s)| {
        if let Some(v) = s.to_owned().pop_back() {
            acc.push((i, v));
        }
        acc
    });
    stacks.sort_by(|(a, _), (b, _)| a.cmp(b));
    stacks.into_iter().map(|(_, v)| v).collect::<String>()
}

pub fn part_2(lines: Vec<String>) -> String {
    move_creates(lines, true)
}
