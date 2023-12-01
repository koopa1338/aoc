use aoc2023::stats;

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut digits = l.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().expect("there should be at least 1 digit");
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

const DIGITS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut line = l.as_bytes();

            let first = 'outer: loop {
                if line[0].is_ascii_digit() {
                    // convert to decimal
                    break line[0].wrapping_sub(b'0') as usize;
                }
                for (value, digit) in DIGITS.iter().enumerate() {
                    if line.starts_with(digit) {
                        break 'outer value + 1;
                    }
                }

                line = &line[1..];
            };

            let last = 'outer: loop {
                let idx = line.len() - 1;
                if line[idx].is_ascii_digit() {
                    // convert to decimal
                    break line[idx].wrapping_sub(b'0') as usize;
                }
                for (value, digit) in DIGITS.iter().enumerate() {
                    if line.ends_with(digit) {
                        break 'outer value + 1;
                    }
                }

                line = &line[..idx];
            };

            10 * first + last
        })
        .sum()
}

pub fn run() {
    let input = include_str!("../input/day1/input.txt");

    println!("DAY 1:");
    stats(|| part_one(input), 1);
    stats(|| part_two(input), 2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        assert_eq!(142, part_one(input));
    }

    #[test]
    fn test_part_two() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(281, part_two(input))
    }
}
