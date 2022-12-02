#[derive(Clone)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

#[derive(Clone, Debug)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl From<&str> for Choice {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Outcome {
    fn from(input: &str) -> Self {
        match input {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Game {
    opponent: Choice,
    me: Choice,
    outcome: Outcome,
}

type Score = usize;

impl Game {
    fn play(self) -> Score {
        match (self.me, self.opponent) {
            (guess @ Choice::Rock, Choice::Scissor)
            | (guess @ Choice::Paper, Choice::Rock)
            | (guess @ Choice::Scissor, Choice::Paper) => guess as Score + Outcome::Win as Score,
            (guess @ Choice::Rock, Choice::Paper)
            | (guess @ Choice::Paper, Choice::Scissor)
            | (guess @ Choice::Scissor, Choice::Rock) => guess as Score + Outcome::Lose as Score,
            (guess @ Choice::Rock, Choice::Rock)
            | (guess @ Choice::Paper, Choice::Paper)
            | (guess @ Choice::Scissor, Choice::Scissor) => guess as Score + Outcome::Draw as Score,
        }
    }

    fn play_predicted(&mut self) -> Score {
        match (&self.opponent, &self.outcome) {
            (Choice::Rock, Outcome::Win) => self.me = Choice::Paper,
            (Choice::Paper, Outcome::Win) => self.me = Choice::Scissor,
            (Choice::Scissor, Outcome::Win) => self.me = Choice::Rock,
            (op, Outcome::Draw) => self.me = op.clone(),
            (Choice::Rock, Outcome::Lose) => self.me = Choice::Scissor,
            (Choice::Paper, Outcome::Lose) => self.me = Choice::Rock,
            (Choice::Scissor, Outcome::Lose) => self.me = Choice::Paper,
        }

        self.clone().play()
    }
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let mut guesses = line.splitn(2, ' ');

        let opponent: Choice = guesses.next().unwrap().into();
        let tmp = guesses.next().unwrap();
        let me: Choice = tmp.into();
        let outcome: Outcome = tmp.into();

        Self {
            opponent,
            me,
            outcome,
        }
    }
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let game: Game = line.trim().into();
            game.play()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let mut game: Game = line.trim().into();
            game.play_predicted()
        })
        .sum()
}

pub fn run() {
    let input = include_str!("../input/day2/input.txt");

    println!("DAY 2:");
    println!("Part 1:");
    println!("{}", part_one(input));

    println!("Part 2:");
    println!("{}", part_two(input));
    println!();
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn test_part_one() {
        assert_eq!(15, part_one(TEST_DATA));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(12, part_two(TEST_DATA))
    }
}
