type Calories = usize;

fn part_one(input: &str) -> Calories {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| {
                    calories
                        .parse::<Calories>()
                        .expect("Error parsing calories.")
                })
                .sum()
        })
        .max()
        .expect("No max calories found for elves.")
}

fn part_two(input: &str) -> Calories {
    let mut elves = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| {
                    calories
                        .parse::<Calories>()
                        .expect("Error parsing calories.")
                })
                .sum()
        })
        .collect::<Vec<Calories>>();
    elves.sort_unstable();

    elves.iter().rev().take(3).sum()
}

pub fn run() {
    let input = include_str!("../input/day1/input.txt");

    println!("DAY 1:");
    println!("Part 1:");
    println!("Max: {}", part_one(input));

    println!("Part 2:");
    println!("Total of top 3: {}", part_two(input));
    println!();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_part_one() {
        assert_eq!(24_000, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(45_000, part_two(INPUT));
    }
}
