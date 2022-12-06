use aoc2022::timing;

fn get_priority(c: char) -> usize {
    let offset = if c.is_ascii_lowercase() { 96 } else { 38 };
    (c as u8 - offset) as usize
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim().split_at(line.len() / 2))
        .map(|(first, second)| {
            let mut filtered = first
                .chars()
                .filter(|c| second.chars().any(|c2| *c == c2))
                .collect::<Vec<char>>();
            filtered.dedup();
            filtered.into_iter().map(get_priority).sum::<usize>()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();

    lines
        .chunks(3)
        .map(|group| {
            let rucksacks = group
                .iter()
                .map(|items| items.chars().map(get_priority).collect::<Vec<usize>>())
                .collect::<Vec<Vec<usize>>>();
            let mut p = 0;
            for prio in 1..=52 {
                if rucksacks.iter().all(|r| r.contains(&prio)) {
                    p = prio;
                    break;
                }
            }
            p
        })
        .sum()
}

pub fn run() {
    let input = include_str!("../input/day3/input.txt");

    println!("DAY 3:");
    timing(|| part_one(input), 1);
    timing(|| part_two(input), 2);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_part_one() {
        assert_eq!(157, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(70, part_two(INPUT));
    }
}
