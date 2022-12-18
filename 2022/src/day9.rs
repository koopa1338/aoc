use aoc2022::timing;
use std::collections::HashSet;

struct Move<const N: usize> {
    distance_x: i32,
    distance_y: i32,
    length: usize,
}

impl<const N: usize> Move<N> {
    fn simulating_rope(
        &self,
        visited: &mut HashSet<(i32, i32)>,
        rope: &mut Vec<(i32, i32)>,
    ) {
        for _ in 0..self.length {
            rope[0] = (rope[0].0 + self.distance_x, rope[0].1 + self.distance_y);
            for i in 1..rope.len() {
                let (dx, dy) = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                if dx.abs() > 1 || dy.abs() > 1 {
                    rope[i].0 += dx.signum();
                    rope[i].1 += dy.signum();
                }
            }
            visited.insert(rope[N]);
        }
    }
}

impl<const N: usize> From<&str> for Move<N> {
    fn from(value: &str) -> Self {
        let (a, b) = value.split_once(' ').unwrap();
        let (distance_x, distance_y) = match a.as_bytes()[0] as char {
            'U' => (0, 1),
            'D' => (0, -1),
            'R' => (1, 0),
            'L' => (-1, 0),
            _ => unreachable!(),
        };
        Move {
            distance_x,
            distance_y,
            length: b.parse::<usize>().unwrap(),
        }
    }
}

fn get_moves<const N: usize>(input: &str) -> Vec<Move<N>> {
    input.lines().map(Into::into).collect()
}

fn part_one(input: &str) -> usize {
    let moves = get_moves::<1>(input);
    let mut rope = vec![(0i32, 0i32); 2];
    let mut visited = HashSet::with_capacity(10000);
    for m in moves {
        m.simulating_rope(&mut visited, &mut rope);
    }
    visited.len()
}

fn part_two(input: &str) -> usize {
    let moves = get_moves::<9>(input);
    let mut rope = vec![(0i32, 0i32); 10];
    let mut visited = HashSet::with_capacity(10000);
    for m in moves {
        m.simulating_rope(&mut visited, &mut rope);
    }
    visited.len()
}

pub fn run() {
    let input = include_str!("../input/day9/input.txt");

    println!("DAY 9:");
    timing(|| part_one(input), 1);
    timing(|| part_two(input), 2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

        assert_eq!(13, part_one(input));
    }

    #[test]
    fn test_part_two() {
        let input: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
        assert_eq!(36, part_two(input));
    }
}
