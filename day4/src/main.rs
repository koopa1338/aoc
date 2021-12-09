#[derive(Debug)]
struct Bingoboard {
    data: Vec<Vec<(u8, bool)>>,
}

impl Bingoboard {
    fn change_state_by_number(&mut self, number: u8) {
        for row in self.data.iter_mut() {
            for idx in row.iter_mut() {
                idx.1 = idx.0 == number;
            }
        }
    }

    fn check_bingo(&mut self) -> bool {
        todo!()
    }

    fn score(&mut self) -> usize {
        let mut score = 0;
        for row in self.data.iter() {
            let row_score = row.into_iter().fold(0, |sum, elem| {
                if elem.1 {
                    return sum + elem.0;
                }
                sum
            });
            score += row_score;
        }

        score.into()
    }
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
                    .map(|p| (p.parse::<u8>().unwrap(), false))
                    .collect::<Vec<(u8, bool)>>()
            }).collect::<Vec<Vec<(u8, bool)>>>()
        }).map(|parsed_chunk| {
            Bingoboard { data: parsed_chunk }
        }).collect();
    (game_numbers, bingoboards)
}


fn part_one(game_numbers: &Vec<u8>, boards: &mut Vec<Bingoboard>) -> Result<usize, String> {
    for (idx, num) in game_numbers.into_iter().enumerate() {
        for board in boards.iter_mut() {
            board.change_state_by_number(*num);
            if idx < 5 {
                // bingo can only occure if we already had 5 numbers
                continue;
            }

            if board.check_bingo() {
                return Ok(board.score());
            }
        }
    }
    Err(String::from("Something went wrong!"))
}

fn main() {
    let (game_numbers, mut bingoboards) = parse_input(include_str!("../input.txt"));

    println!("Part 1:");
    let score = part_one(&game_numbers, &mut bingoboards);
    println!("Score of the winner board:\n{}", score.unwrap());

    println!("Part 2:");
}

#[cfg(test)]
mod tests {
    use super::*;
}
