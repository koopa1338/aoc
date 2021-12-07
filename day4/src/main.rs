#[derive(Debug)]
struct Bingoboard {
    data: Vec<Vec<u8>>,
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<Bingoboard>) {
    let mut lines = input.lines().collect::<Vec<&str>>();

    let game_numbers: Vec<u8> = lines
        .first()
        .unwrap()
        .split(',')
        .into_iter()
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let bingoboards = lines
        .split_off(2)
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
        .chunks(5)
        .map(|chunk| {
            chunk.iter().map(|c| {
                c.split(' ')
                    .into_iter()
                    .map(|value| value.trim())
                    .filter(|x| !x.is_empty())
                    .map(|p| p.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            }).collect::<Vec<Vec<u8>>>()
        }).map(|parsed_chunk| {
            Bingoboard { data: parsed_chunk }
        }).collect();
    println!("Bingoboards: {:?}", bingoboards);

    (game_numbers, bingoboards)
}

fn main() {
    let (game_numbers, bingoboards) = parse_input(include_str!("../input.txt"));

    println!("Part 1:");

    println!("Part 2:");
}

#[cfg(test)]
mod tests {
    use super::*;
}
