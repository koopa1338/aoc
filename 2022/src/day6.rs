use aoc2022::stats;

fn find_packet(bytes: &[u8], marker_size: usize) -> usize {
    bytes
        .windows(marker_size)
        .enumerate()
        .find(|(_, window)| {
            // this is one order of magnitued slower ...
            // window.iter().collect::<HashSet<_>>().len() == 4
            let mut values = window.to_vec();
            values.sort_unstable();
            values.dedup();
            values.len() == marker_size
        })
        .map(|(marker_idx, _)| marker_idx + marker_size)
        .expect("no solution found")
}

fn part_one(input: &str) -> usize {
    find_packet(input.as_bytes(), 4)
}

fn part_two(input: &str) -> usize {
    find_packet(input.as_bytes(), 14)
}

pub fn run() {
    let input = include_str!("../input/day6/input.txt");

    println!("DAY 6:");
    stats(|| part_one(input), 1);
    stats(|| part_two(input), 2);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, part_one(input));
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_part_two(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, part_two(input));
    }
}
