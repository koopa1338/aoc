use aoc2024::stats;

fn part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|value| value.parse::<u32>().ok())
                .collect()
        })
        .map(|elements: Vec<u32>| is_safe(&elements))
        .filter(|&safe| safe)
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|value| value.parse::<u32>().ok())
                .collect()
        })
        .map(|elements: Vec<u32>| {
            if is_safe(&elements) {
                true
            } else {
                let mut variants = (0..elements.len()).map(|i| {
                    elements
                        .iter()
                        .enumerate()
                        .filter(move |(j, _)| i != *j)
                        .map(|(_, p)| *p)
                });

                variants.any(|variant| is_safe(&variant.collect::<Vec<u32>>()))
            }
        })
        .filter(|&safe| safe)
        .count()
}

fn is_safe(reports: &[u32]) -> bool {
    if !reports.is_sorted() && !reports.is_sorted_by(|a, b| a >= b) || reports.len() < 2 {
        false
    } else {
        reports
            .windows(2)
            .map(|p| p[0].abs_diff(p[1]))
            .all(|d| (1..=3).contains(&d))
    }
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

    #[test]
    fn example() {
        let input = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let expected = 2;
        assert_eq!(part_one(input), expected);
    }

    #[test]
    fn example_part_two() {
        let input = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let expected = 4;
        assert_eq!(part_two(input), expected);
    }
}
