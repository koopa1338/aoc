#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Debug)]
struct Line {
    start: (u32, u32),
    end: (u32, u32),
    direction: Direction,
}

const SIZE: usize = 1000;

fn parse_input(input: &str) -> Result<Vec<Line>, String> {
    Ok(input
        .lines()
        .into_iter()
        .map(|s| {
            let coords = s
                .split(' ')
                .filter(|elem| elem.contains(","))
                .collect::<Vec<&str>>();
            let start = coords[0]
                .split(",")
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let end = coords[1]
                .split(",")
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let mut direction = Direction::Diagonal;
            if start[1] == end[1] {
                direction = Direction::Horizontal;
            }
            if start[0] == end[0] {
                direction = Direction::Vertical;
            }

            Line {
                start: (start[0], start[1]),
                end: (end[0], end[1]),
                direction,
            }
        })
        .collect::<Vec<Line>>())
}

fn part_one(lines: Vec<Line>) -> Result<usize, String> {
    let mut diagram = vec![vec![0; SIZE]; SIZE];
    let vert_and_horiz_lines = lines
        .into_iter()
        .filter(|line| match line.direction {
            Direction::Diagonal => false,
            _ => true,
        })
        .collect::<Vec<Line>>();
    for line in vert_and_horiz_lines {
        match line.direction {
            Direction::Horizontal => {
                let y = line.start.1;
                let mut range = line.start.0..=line.end.0;
                if line.start.0 > line.end.0 {
                    range = line.end.0..=line.start.0;
                }
                for x in range {
                    diagram[y as usize][x as usize] += 1;
                }
            }
            Direction::Vertical => {
                let x = line.start.0;
                let mut range = line.start.1..=line.end.1;
                if line.start.1 > line.end.1 {
                    range = line.end.1..=line.start.1;
                }
                for y in range {
                    diagram[y as usize][x as usize] += 1;
                }
            }
            _ => continue,
        }
    }
    let danger_count = diagram.into_iter().flatten().filter(|d| *d >= 2).count();
    Ok(danger_count)
}

fn part_two() -> Result<(), String> {
    Ok(())
}

fn main() {
    let lines = parse_input(include_str!("../input.txt")).unwrap();

    println!("Part 1:");
    println!("Number of dangerous areas: {}", part_one(lines).unwrap());

    // println!("Part 2:");
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
        assert_eq!(lines.len(), 10);
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
        let input_string = input.join("\n");
        let lines = parse_input(&input_string).unwrap();
        assert_eq!(part_one(lines).unwrap(), 5);
    }

    // #[test]
    // fn test_part_two() {
    // }
}
