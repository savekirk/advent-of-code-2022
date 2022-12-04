/**
 * Opponent
 * A - Rock
 * B - Paper
 * C - Scissors
 *
 * You
 * X - Rock
 * Y - Paper
 * Z - Scissors
 *
 * Part 2
 * X - lose
 * Y - draw
 * Z - win
 *
 * Scores
 * Shape Score
 * Rock - 1
 * Paper - 2
 * Scissors - 3
 *
 * Outcome score
 * Win - 6
 * Draw - 3
 * Lose - 0
 *
 * Final score
 * Shape score + Outcome score
 */

#[derive(Debug, Clone, Copy)]
struct Strategy {
    col1: i32,
    col2: i32,
}

impl Strategy {
    fn new(opponent: char, you: char) -> Self {
        let to_digit = |c: char| c.to_digit(36).unwrap();
        let o = (to_digit(opponent) % 10) + 1;
        let y = (to_digit(you) % 33) + 1;
        Self {
            col1: o as i32,
            col2: y as i32,
        }
    }

    fn outcome_score(self, is_part_1: bool) -> i32 {
        match (self.to_play(is_part_1) - self.col1) as i32 {
            -2 | 1 => 6,
            0 => 3,
            _ => 0,
        }
    }

    fn to_play(self, is_part_1: bool) -> i32 {
        if is_part_1 {
            self.col2
        } else {
            match self.col2 {
                1 => [1, 2, 3].iter().fold(1, |tp, v| match v - self.col1 {
                    -2 | 1 | 0 => tp,
                    _ => *v,
                }),
                2 => self.col1,
                _ => [1, 2, 3].iter().fold(1, |tp, v| match v - self.col1 {
                    2 | -1 | 0 => tp,
                    _ => *v,
                }),
            }
        }
    }

    fn score(self) -> i32 {
        self.col2 + self.outcome_score(true)
    }

    fn part_2_score(self) -> i32 {
        self.to_play(false) + self.outcome_score(false)
    }
}

pub fn part_1(lines: Vec<String>) -> i32 {
    let strategies = get_strategies(lines);

    strategies.iter().fold(0, |mut acc, s| {
        acc = acc + s.score();
        return acc;
    })
}

pub fn part_2(lines: Vec<String>) -> i32 {
    let strategies = get_strategies(lines);

    strategies.iter().fold(0, |mut acc, s| {
        acc = acc + s.part_2_score();
        return acc;
    })
}

fn get_strategies(lines: Vec<String>) -> Vec<Strategy> {
    lines.iter().fold(Vec::new(), |mut acc, line| {
        let (opponent, you) = line
            .split_once(" ")
            .map(|(o, y)| (o.parse::<char>().unwrap(), y.parse::<char>().unwrap()))
            .unwrap();
        acc.push(Strategy::new(opponent, you));
        return acc;
    })
}
