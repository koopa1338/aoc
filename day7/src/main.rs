
fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<usize>().unwrap()
        })
        .collect::<Vec<usize>>()
}

fn part_one(mut positions: Vec<usize>) -> usize {
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
        sum + distance.abs() as usize
    })
}

#[allow(dead_code)]
fn part_two(_positions: Vec<usize>) -> usize {
    unimplemented!();
}

fn main() {
    let input = include_str!("../input.txt");
    let data = parse_input(input);
    println!("Part 1:");
    println!("fuel needed: {}", part_one(data.clone()));

    println!("Part 2:");
    println!("fuel needed: {}", part_one(data));
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

    /* #[test]
    fn test_part_two() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let data = parse_input(input);
        let fuel = part_two(data);

        assert_eq!(168, fuel);
    } */
}
