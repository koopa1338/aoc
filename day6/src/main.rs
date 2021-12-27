#[derive(Clone, Copy, Debug)]
struct Lanternfish {
    timer: u8,
}

fn parse_input(input: &str) -> Vec<Lanternfish> {
    input
        .trim()
        .split(',')
        .into_iter()
        .map(|value| Lanternfish {
            timer: value.parse::<u8>().unwrap(),
        })
        .collect::<Vec<Lanternfish>>()
}

fn part_one(fishes: &mut Vec<Lanternfish>, iterations: usize) -> usize {
    for _ in 0..iterations {
        let mut new_fishes: Vec<Lanternfish> = Vec::new();
        fishes.into_iter().for_each(|mut fish| match fish.timer {
            0 => {
                fish.timer = 6;
                new_fishes.push(Lanternfish { timer: 8 })
            }
            _ => fish.timer -= 1,
        });
        fishes.append(&mut new_fishes);
        new_fishes.clear();
    }

    fishes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let input = "3,4,3,1,2";
        let parsed = parse_input(input);

        assert_eq!(parsed.len(), 5);
    }

    #[test]
    fn test_part_one() {
        let input = "3,4,3,1,2";
        let mut parsed = parse_input(input);

        assert_eq!(part_one(&mut parsed, 18), 26);
    }
}

fn main() {
    let mut fishes = parse_input(include_str!("../input.txt"));
    let iterations = 80;
    println!("Part 1:");
    println!(
        "fishes after {} iterations: {}",
        iterations,
        part_one(&mut fishes, iterations)
    );
    println!("Part 2:");
}
