use aoc2022::timing;

fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|(first_low, first_high, second_low, second_high)| {
            (first_low >= second_low && first_high <= second_high)
                || (first_low <= second_low && first_high >= second_high)
        })
        .count()
}

fn part_two(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|(first_low, first_high, second_low, second_high)| {
            (first_low >= second_low && first_low <= second_high)
                || (first_high >= second_low && first_high <= second_high)
                || (first_low <= second_low && first_high >= second_low)
                || (first_low <= second_high && first_high >= second_high)
        })
        .count()
}

fn parse_input(input: &str) -> Vec<(u8, u8, u8, u8)> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let ((first_low, first_high), (second_low, second_high)) = (
                first.split_once('-').unwrap(),
                second.split_once('-').unwrap(),
            );

            (
                first_low.parse::<u8>().unwrap(),
                first_high.parse::<u8>().unwrap(),
                second_low.parse::<u8>().unwrap(),
                second_high.parse::<u8>().unwrap(),
            )
        })
        .collect()
}

pub fn run() {
    let input = include_str!("../input/day4/input.txt");

    println!("DAY 4:");
    timing(|| part_one(input), 1);
    timing(|| part_two(input), 2);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_part_one() {
        assert_eq!(2, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(4, part_two(INPUT));
    }
}
