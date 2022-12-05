#[derive(Debug)]
struct Crates {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
struct ProcedureStep {
    count: usize,
    from: usize,
    to: usize,
}

fn part_one(input: &str) -> String {
    let (mut crates, procedures) = parse_input(input);

    for step in procedures {
        for _ in 0..step.count {
            if let Some(c) = crates.stacks[step.from].pop() {
                crates.stacks[step.to].push(c);
            }
        }
    }

    crates
        .stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop())
        .collect()
}

fn part_two(input: &str) -> String {
    let (mut crates, procedures) = parse_input(input);

    for step in procedures {
        let mut tmp = Vec::new();
        for _ in 0..step.count {
            if let Some(c) = crates.stacks[step.from].pop() {
                tmp.push(c);
            }
        }
        tmp.reverse();
        crates.stacks[step.to].append(&mut tmp);
    }

    crates
        .stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop())
        .collect()
}

fn parse_input(input: &str) -> (Crates, Vec<ProcedureStep>) {
    let (crates_str, procedures_str) = input.split_once("\n\n").expect("invalid input");
    let mut crates_iter = crates_str.lines().rev();
    let dim = crates_iter.next().unwrap().split_whitespace().count();

    let mut stacks = vec![Vec::new(); dim as usize];
    crates_iter.for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(idx, c)| {
                if !c.is_whitespace() {
                    stacks[idx].push(c);
                }
            });
    });

    let crates = Crates { stacks };
    let procedures = procedures_str
        .trim()
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["move", count, "from", from, "to", to] => ProcedureStep {
                    count: count.parse::<usize>().unwrap(),
                    from: from.parse::<usize>().unwrap().checked_sub(1).unwrap(),
                    to: to.parse::<usize>().unwrap().checked_sub(1).unwrap(),
                },
                _ => unreachable!(),
            },
        )
        .collect::<Vec<_>>();

    (crates, procedures)
}

pub fn run() {
    let input = include_str!("../input/day5/input.txt");

    println!("DAY 5:");
    println!("Part 1:");
    println!("{}", part_one(input));

    println!("Part 2:");
    println!("{}", part_two(input));
    println!();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_part_one() {
        assert_eq!("CMZ", part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!("MCD", part_two(INPUT));
    }
}
