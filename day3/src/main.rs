fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|number| number.to_digit(2).unwrap())
                .collect()
        })
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

fn get_most_and_least_bits(matrix: Vec<Vec<u32>>) -> (Vec<u32>, Vec<u32>) {
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

    (most_common_vec, least_common_vec)
}

fn decode_part_one(matrix: Vec<Vec<u32>>) -> (usize, usize) {
    let trans = matrix_transpose(matrix);
    let (most_common_vec, least_common_vec) = get_most_and_least_bits(trans);
    let most_common = convert_bits_to_usize(most_common_vec);
    let least_common = convert_bits_to_usize(least_common_vec);
    (most_common, least_common)
}

fn decode_part_two(matrix: Vec<Vec<u32>>) -> (usize, usize) {
    let mut oxygen: Vec<Vec<u32>> = matrix.clone();
    let mut carbon: Vec<Vec<u32>> = matrix.clone();
    let bit_len = matrix.first().unwrap().len();

    for idx in 0..bit_len {
        // println!("co: {:?}", &carbon);
        let (most, _) = get_most_and_least_bits(matrix_transpose(oxygen.clone()));
        let (_, least) = get_most_and_least_bits(matrix_transpose(carbon.clone()));
        println!("idx: {}, most: {:?}, ox: {:?}", idx, most, &oxygen);

        oxygen = oxygen
            .into_iter()
            .filter(|v| *v.get(idx).unwrap() == *most.get(idx).unwrap())
            .collect();
        carbon = carbon
            .into_iter()
            .filter(|v| *v.get(idx).unwrap() == *least.get(idx).unwrap())
            .collect();
    }

    if oxygen.len() != 1 {
        panic!("There should only be one value left");
    }
    if carbon.len() != 1 {
        panic!("There should only be one value left");
    }

    let oxygen_as_usize = convert_bits_to_usize(oxygen.first().unwrap().to_vec());
    let carbon_as_usize = convert_bits_to_usize(carbon.first().unwrap().to_vec());

    (oxygen_as_usize, carbon_as_usize)
}

fn convert_bits_to_usize(bits: Vec<u32>) -> usize {
    usize::from_str_radix(
        &bits
            .iter()
            .map(|bit| bit.to_string())
            .collect::<Vec<String>>()
            .concat(),
        2,
    )
    .unwrap()
}

fn main() {
    let input: Vec<Vec<u32>> = parse_input(include_str!("../input.txt"));
    println!("Part 1:");
    let (gamma, epsilon) = decode_part_one(input);
    let power_comsumption = gamma * epsilon;
    println!(
        "gamma: {}; epsilon: {}; power comsumption: {}",
        gamma, epsilon, power_comsumption
    );

    println!("Part 2:");
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
        let (gamma, epsilon) = decode_part_one(parsed);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn test_decode_part_two() {
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
        let (oxygen, carbon) = decode_part_two(parsed);
        assert_eq!(oxygen, 23);
        assert_eq!(carbon, 10);
    }
}
