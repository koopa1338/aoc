use aoc2022::stats;

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
    outcome: Option<Outcome>,
}

type Score = usize;

impl Game {
    fn play(&mut self) {
        self.outcome = match (&self.me, &self.opponent) {
            (Choice::Rock, Choice::Scissor)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissor, Choice::Paper) => Some(Outcome::Win),
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissor)
            | (Choice::Scissor, Choice::Rock) => Some(Outcome::Lose),
            (Choice::Rock, Choice::Rock)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Scissor, Choice::Scissor) => Some(Outcome::Draw),
        };
    }

    fn play_predicted(&mut self) {
        self.me = match (&self.opponent, &self.outcome) {
            (Choice::Rock, Some(Outcome::Win)) | (Choice::Scissor, Some(Outcome::Lose)) => {
                Choice::Paper
            }
            (Choice::Paper, Some(Outcome::Win)) | (Choice::Rock, Some(Outcome::Lose)) => {
                Choice::Scissor
            }
            (Choice::Scissor, Some(Outcome::Win)) | (Choice::Paper, Some(Outcome::Lose)) => {
                Choice::Rock
            }
            (op, Some(Outcome::Draw)) => op.clone(),
            _ => unreachable!(),
        };
    }

    fn score(&self) -> Score {
        self.outcome
            .clone()
            .map(|outcome| self.me.clone() as usize + outcome as usize)
            .expect("could not calculate score")
    }
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let mut guesses = line.splitn(2, ' ');

        let opponent: Choice = guesses.next().unwrap().into();
        let tmp = guesses.next().unwrap();
        let me: Choice = tmp.into();
        let outcome: Option<Outcome> = Some(tmp.into());

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
            let mut game: Game = line.trim().into();
            game.play();
            game.score()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let mut game: Game = line.trim().into();
            game.play_predicted();
            game.score()
        })
        .sum()
}

pub fn run() {
    let input = include_str!("../input/day2/input.txt");

    println!("DAY 2:");
    stats(|| part_one(input), 1);
    stats(|| part_two(input), 2);
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
