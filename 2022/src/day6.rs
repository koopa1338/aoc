use aoc2022::timing;

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
    timing(|| part_one(input), 1);
    timing(|| part_two(input), 2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut stream = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, part_one(stream));

        stream = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, part_one(stream));

        stream = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, part_one(stream));

        stream = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(10, part_one(stream));

        stream = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(11, part_one(stream));
    }

    #[test]
    fn test_part_two() {
        let mut stream = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, part_two(stream));

        stream = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(23, part_two(stream));

        stream = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(23, part_two(stream));

        stream = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(29, part_two(stream));

        stream = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(26, part_two(stream));
    }
}
