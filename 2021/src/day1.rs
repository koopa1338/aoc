use aoc2021::timing;

fn part1(input: Vec<usize>) -> usize {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

fn part2(input: Vec<usize>) -> usize {
    let sums = input
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<usize>>();
    part1(sums)
}

pub fn run() {
    let mut input: Vec<usize> = include_str!("../input/day1/part1.txt")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("DAY 1:");
    timing(|| part1(input), 1);

    input = include_str!("../input/day1/part2.txt")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    timing(|| part2(input), 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn test_part2() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part2(input), 5);
    }
}
