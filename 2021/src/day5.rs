use aoc2021::stats;
use itertools::Either;

#[derive(Debug, Clone)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Direction {
    fn get_diagonal_iter(start: u32, end: u32) -> impl Iterator<Item = u32> {
        if start < end {
            return Either::Left(start..=end);
        }
        Either::Right((end..=start).rev())
    }
}

#[derive(Debug, Clone)]
struct Line {
    start: (u32, u32),
    end: (u32, u32),
    direction: Direction,
}

impl Line {
    fn get_diagram_coords(&self, diagram: &mut Vec<Vec<usize>>) {
        match self.direction {
            Direction::Horizontal => {
                let y = self.start.1;
                let mut range = self.start.0..=self.end.0;
                if self.start.0 > self.end.0 {
                    range = self.end.0..=self.start.0;
                }
                for x in range {
                    diagram[y as usize][x as usize] += 1;
                }
            }
            Direction::Vertical => {
                let x = self.start.0;
                let mut range = self.start.1..=self.end.1;
                if self.start.1 > self.end.1 {
                    range = self.end.1..=self.start.1;
                }
                for y in range {
                    diagram[y as usize][x as usize] += 1;
                }
            }
            Direction::Diagonal => {
                let iter_x = Direction::get_diagonal_iter(self.start.0, self.end.0);
                let iter_y = Direction::get_diagonal_iter(self.start.1, self.end.1);
                for (x, y) in iter_x.zip(iter_y) {
                    diagram[y as usize][x as usize] += 1;
                }
            }
        }
    }
}

const SIZE: usize = 1000;

fn parse_input(input: &str) -> Result<Vec<Line>, String> {
    Ok(input
        .lines()
        .into_iter()
        .map(|s| {
            let coords = s
                .split(' ')
                .filter(|elem| elem.contains(','))
                .collect::<Vec<&str>>();
            let start = coords[0]
                .split(',')
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let end = coords[1]
                .split(',')
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
        .filter(|line| !matches!(line.direction, Direction::Diagonal))
        .collect::<Vec<Line>>();
    for line in vert_and_horiz_lines {
        line.get_diagram_coords(&mut diagram);
    }
    let danger_count = diagram.into_iter().flatten().filter(|d| *d >= 2).count();
    Ok(danger_count)
}

fn part_two(lines: Vec<Line>) -> Result<usize, String> {
    let mut diagram = vec![vec![0; SIZE]; SIZE];
    for line in lines {
        line.get_diagram_coords(&mut diagram);
    }
    let danger_count = diagram.into_iter().flatten().filter(|d| *d >= 2).count();
    Ok(danger_count)
}

pub fn run() {
    let lines = parse_input(include_str!("../input/day5/input.txt")).unwrap();

    println!("DAY 5:");
    stats(|| part_one(lines.clone()).unwrap(), 1);
    stats(|| part_two(lines).unwrap(), 2);
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

    #[test]
    fn test_part_two() {
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
        assert_eq!(part_two(lines).unwrap(), 12);
    }
}

