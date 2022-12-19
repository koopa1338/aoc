use aoc2022::stats;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Crates {
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn get_crates_top(&mut self) -> String {
        self.clone()
            .stacks
            .into_iter()
            .filter_map(|mut stack| stack.pop())
            .collect()
    }
}

impl From<&str> for Crates {
    fn from(input: &str) -> Self {
        let mut crates_iter = input.lines().rev();
        let dim = crates_iter.next().unwrap().split_whitespace().count();

        let mut stacks = vec![Vec::new(); dim as usize];
        for line in crates_iter {
            for (idx, character) in line.chars().skip(1).step_by(4).enumerate() {
                if !character.is_whitespace() {
                    stacks[idx].push(character);
                }
            }
        }

        Self { stacks }
    }
}

#[derive(Debug)]
struct ProcedureStep {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for ProcedureStep {
    fn from(input: &str) -> Self {
        match input.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["move", count, "from", from, "to", to] => Self {
                count: count.parse::<usize>().unwrap(),
                from: from.parse::<usize>().unwrap().checked_sub(1).unwrap(),
                to: to.parse::<usize>().unwrap().checked_sub(1).unwrap(),
            },
            _ => unreachable!(),
        }
    }
}

fn part_one(input: &str) -> String {
    let (mut crates, procedures) = parse_input(input);

    for step in procedures {
        for _ in 0..step.count {
            if let Some(c) = crates.stacks[step.from].pop() {
                crates.stacks[step.to].push(c);
            }
        }
    }

    crates.get_crates_top()
}

fn part_two(input: &str) -> String {
    let (mut crates, procedures) = parse_input(input);

    for step in procedures {
        let mut tmp = VecDeque::new();
        for _ in 0..step.count {
            if let Some(c) = crates.stacks[step.from].pop() {
                tmp.push_front(c);
            }
        }
        crates.stacks[step.to].extend(tmp);
    }

    crates.get_crates_top()
}

fn parse_input(input: &str) -> (Crates, Vec<ProcedureStep>) {
    let (crates_str, procedures_str) = input.split_once("\n\n").expect("invalid input");
    let mut crates_iter = crates_str.lines().rev();
    let dim = crates_iter.next().unwrap().split_whitespace().count();

    let mut stacks = vec![Vec::new(); dim as usize];
    for line in crates_iter {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(idx, c)| {
                if !c.is_whitespace() {
                    stacks[idx].push(c);
                }
            });
    }

    let crates = Crates::from(crates_str);
    let procedures = procedures_str
        .trim()
        .lines()
        .map(ProcedureStep::from)
        .collect::<Vec<_>>();

    (crates, procedures)
}

pub fn run() {
    let input = include_str!("../input/day5/input.txt");

    println!("DAY 5:");
    stats(|| part_one(input), 1);
    stats(|| part_two(input), 2);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_part_one() {
        assert_eq!("CMZ", part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!("MCD", part_two(INPUT));
    }
}
