use aoc2021::stats;

use std::{
    ops::Mul,
    str::{FromStr, Lines},
};

#[derive(Debug, Clone, Copy)]
struct Command {
    cmd: Instruction,
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Forward,
    Down,
    Up,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        match input {
            "forward" => Ok(Instruction::Forward),
            "down" => Ok(Instruction::Down),
            "up" => Ok(Instruction::Up),
            _ => Err(()),
        }
    }
}

fn parse_input(input: Lines) -> Vec<Command> {
    input
        .map(|line| {
            let split: Vec<&str> = line.split(' ').collect();
            Command {
                cmd: Instruction::from_str(split[0]).unwrap(),
                value: split[1].parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<Command>>()
}

fn part1(data: Vec<Command>) -> i32 {
    let (horizontal, vertical): (Vec<Command>, Vec<Command>) =
        data.iter().partition(|command| match command.cmd {
            Instruction::Forward => true,
            Instruction::Down => false,
            Instruction::Up => false,
        });

    let horizontal_pos: i32 = horizontal.iter().map(|command| command.value).sum();

    let vertical_pos: i32 = vertical
        .iter()
        .map(|command| match command.cmd {
            Instruction::Up => command.value.saturating_neg(),
            _ => command.value,
        })
        .sum();

    horizontal_pos.mul(vertical_pos)
}

fn part2(data: Vec<Command>) -> i32 {
    let (mut h, mut d, mut a) = (0, 0, 0);
    for command in data.iter() {
        match command.cmd {
            Instruction::Forward => {
                h += command.value;
                d += a.mul(command.value);
            }
            Instruction::Down => a += command.value,
            Instruction::Up => a -= command.value,
        }
    }
    h.mul(d)
}

pub fn run() {
    let input: Vec<Command> = parse_input(include_str!("../input/day2/input.txt").lines());

    println!("DAY 2:");
    stats(|| part1(input.clone()), 1);
    stats(|| part2(input), 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_to_commands() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let parsed = parse_input(input.lines());
        assert_eq!(parsed.len(), 6);

        let last = parsed.last().unwrap();
        assert_eq!(last.value, 2);
    }

    #[test]
    fn calc_cource_version_one() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let parsed = parse_input(input.lines());

        assert_eq!(150, part1(parsed));
    }

    #[test]
    fn calc_cource_version_two() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let parsed = parse_input(input.lines());

        assert_eq!(900, part2(parsed));
    }
}
