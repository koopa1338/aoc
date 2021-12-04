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
fn decode_part_one(matrix: Vec<Vec<u32>>) -> (Vec<u32>, Vec<u32>) {
    let mut most_common_vec: Vec<u32> = Vec::new();
    let mut least_common_vec: Vec<u32> = Vec::new();

    for bit_line in matrix.iter() {
        let common_bit_ones = bit_line.iter().filter(|bit| **bit > 0).count() as u32;
        if common_bit_ones >= bit_line.len() as u32 / 2 {
            most_common_vec.push(1);
            least_common_vec.push(0);
        } else {
            most_common_vec.push(0);
            least_common_vec.push(1);
        }
    }
    // TODO: convert to integer
    (most_common_vec, least_common_vec)
}

fn main() {
    let input: Vec<Vec<u32>> = parse_input(include_str!("../input.txt"));
    let transposed = matrix_transpose(input);
    println!("Part 1:");
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
}

