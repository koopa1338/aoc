struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

fn parse_input(input: &str) -> Result<(), String> {
    let tuples = input
        .split(' ')
        .into_iter()
        .filter(|elements| elements.contains(","))
        .collect::<Vec<&str>>();

    Ok(())
}

fn part_one() -> Result<(), String> {
    Ok(())
}

fn part_two() -> Result<(), String> {
    Ok(())
}

fn main() {
    let test = parse_input(include_str!("../input.txt"));

    println!("Part 1:");

    println!("Part 2:");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let input = &[
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];
    }

    #[test]
    fn test_part_one() {
        let input = &[
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];
    }

    // #[test]
    // fn test_part_two() {
    // }
}
