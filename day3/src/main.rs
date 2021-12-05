fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|l| l.chars()
            .map(|number| number.to_digit(2).unwrap())
            .collect())
        .collect()
}
fn matrix_transpose(matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut trans = vec![Vec::with_capacity(matrix.len()); matrix[0].len()];
    for r in matrix {
        for i in 0..r.len() {
            trans[i].push(r[i]);
        }
    }
    trans
}
fn decode_part_one(matrix: Vec<Vec<u32>>) -> (usize, usize) {
    let mut most_common_vec: Vec<&str> = Vec::new();
    let mut least_common_vec: Vec<&str> = Vec::new();

    for bit_line in matrix.iter() {
        let common_bit_ones = bit_line.iter().filter(|bit| **bit > 0).count() as u32;
        if common_bit_ones >= bit_line.len() as u32 / 2 {
            most_common_vec.push("1");
            least_common_vec.push("0");
        } else {
            most_common_vec.push("0");
            least_common_vec.push("1");
        }
    }
    let most_common = usize::from_str_radix(&most_common_vec.concat(), 2).unwrap();
    let least_common = usize::from_str_radix(&least_common_vec.concat(), 2).unwrap();
    (most_common, least_common)
}

fn main() {
    let input: Vec<Vec<u32>> = parse_input(include_str!("../input.txt"));
    let transposed = matrix_transpose(input);
    println!("Part 1:");
    let (gamma, epsilon) = decode_part_one(transposed);
    let power_comsumption = gamma * epsilon;
    println!("gamma: {}; epsilon: {}; power comsumption: {}", gamma, epsilon, power_comsumption);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
        let parsed = parse_input(input);
        assert_eq!(parsed.len(), 12);
    }

    #[test]
    fn test_decode_part_one() {
        let input: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
        let parsed = parse_input(input);
        let transposed = matrix_transpose(parsed);
        let (gamma, epsilon) = decode_part_one(transposed);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }
}

