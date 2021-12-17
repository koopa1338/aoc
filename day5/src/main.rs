struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {
    fn intersect(self, line: &Line) -> (u32, u32) {
        todo!();
    }
}   

fn parse_input(input: &str) -> Result<Vec<Line>, String> {
    Ok(input
        .lines()
        .into_iter()
        .map(|s| {
            let coords = s.split(' ').filter(|elem| elem.contains(",")).collect::<Vec<&str>>();
            let start = coords[0].split(",").map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            let end = coords[1].split(",").map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            Line {
                start: (start[0], start[1]),
                end: (end[0], end[1]),
            }
        }).collect::<Vec<Line>>())
}

fn part_one(lines: Vec<Line>) -> Result<(), String> {
    Ok(())
}

fn part_two() -> Result<(), String> {
    Ok(())
}

fn main() {
    let test = parse_input(include_str!("../input.txt")).unwrap();

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
        let input_string = input.join("\n");
        let lines = parse_input(&input_string).unwrap();
        assert_eq!(lines.len(), input.len());
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
