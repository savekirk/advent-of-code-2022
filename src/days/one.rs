#[derive(Debug, Clone)]
struct Elf {
    id: i32,
    calories: Vec<i32>,
    total_calories: i32,
}

impl Elf {
    fn new(id: i32) -> Elf {
        Self {
            id,
            calories: Vec::new(),
            total_calories: 0,
        }
    }

    fn update_calory(&mut self, calory: i32) {
        self.calories.push(calory);
        self.total_calories = self.total_calories + calory;
    }
}

pub fn part_1(lines: Vec<String>) -> i32 {
    return get_elves(lines)
        .iter()
        .max_by(|a, b| a.total_calories.cmp(&b.total_calories))
        .unwrap()
        .total_calories;
}

pub fn part_2(lines: Vec<String>) -> i32 {
    let mut elves = get_elves(lines);
    elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));

    return (&elves[0..3])
        .iter()
        .fold(0, |acc, e| acc + e.total_calories);
}

fn get_elves(lines: Vec<String>) -> Vec<Elf> {
    let mut elf = Elf::new(1);
    let mut id = 1;
    let mut elves = Vec::new();

    lines.into_iter().for_each(|line| {
        if line.trim().is_empty() {
            id = id + 1;
            elves.push(elf.clone());
            elf = Elf::new(id);
        } else {
            let calory = line.parse::<i32>().unwrap();
            elf.update_calory(calory);
        }
    });

    return elves;
}
