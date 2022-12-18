use aoc2021::timing;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>()
}

fn part_one(mut positions: Vec<i32>) -> i32 {
    positions.sort_unstable();
    let len = positions.len();
    let idx = if let 0 = len % 2 {
        len / 2
    } else {
        (len + 1) / 2
    };

    let horiz = *positions.get(idx).unwrap();

    positions.iter().fold(0, |sum, val| {
        let distance = horiz as i32 - *val as i32;
        sum + distance.abs()
    })
}

fn gauss(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn part_two(positions: Vec<i32>) -> i32 {
    let avg = positions.iter().sum::<i32>() / positions.len() as i32;
    positions.iter().map(|pos| {
        gauss((pos - avg).abs())
    }).sum()
}

pub fn run() {
    let input = include_str!("../input/day7/input.txt");
    let data = parse_input(input);
    println!("DAY 7:");
    timing(|| part_one(data.clone()), 1);
    timing(|| part_two(data), 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let data = parse_input(input);

        assert_eq!(10, data.len());
    }

    #[test]
    fn test_part_one() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let data = parse_input(input);
        let fuel = part_one(data);

        assert_eq!(37, fuel);
    }

    #[test]
    fn test_part_two() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let data = parse_input(input);
        let fuel = part_two(data);

        assert_eq!(170, fuel);
    }
}

