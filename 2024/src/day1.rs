use aoc2024::stats;

fn part_one(input: &str) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| {
            line.split_once(' ').map(|(left, right)| {
                (
                    left.parse::<i32>().unwrap(),
                    right.trim().parse::<i32>().unwrap(),
                )
            })
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right).map(|(&l, r)| l.abs_diff(r)).sum()
}

fn part_two(input: &str) -> usize {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| {
            line.split_once(' ').map(|(left, right)| {
                (
                    left.parse::<i32>().unwrap(),
                    right.trim().parse::<i32>().unwrap(),
                )
            })
        })
        .unzip();

    left.iter().fold(0, |acc, x| {
        let count = right.iter().filter(|&&val| *x == val).count();
        acc + *x as usize * count
    })
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
    fn example() {
        let input = r"
3   4
4   3
2   5
1   3
3   9
3   3";
        let expected = 11;
        assert_eq!(part_one(input), expected);
    }
}
