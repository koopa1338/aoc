use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Bingoboard {
    data: Vec<Vec<(u8, bool)>>,
    winning_number: Option<u8>,
}

impl Bingoboard {
    fn change_state_by_number(&mut self, number: u8) {
        for row in self.data.iter_mut() {
            for idx in row.iter_mut() {
                let new_state = idx.0 == number;
                if new_state {
                    idx.1 = new_state
                }
            }
        }
    }

    fn check_bingo(&mut self) -> bool {
        //rows
        let bingo_row = self
            .data
            .iter()
            .filter(|row| row.iter().filter(|index| index.1).count() == 5)
            .count()
            == 1;
        //columns
        let trans = self.matrix_transpose();
        let bingo_col = trans
            .iter()
            .filter(|row| row.iter().filter(|index| index.1).count() == 5)
            .count()
            == 1;
        bingo_row || bingo_col
    }

    fn score(&mut self) -> Result<u64, String> {
        if self.winning_number.is_none() {
            return Err(String::from("no winning number for this board."));
        }
        let mut score: u64 = 0;
        for row in self.data.iter() {
            let row_score: u64 = row.iter().fold(0, |sum, elem| {
                if !elem.1 {
                    return sum as u64 + elem.0 as u64;
                }
                sum as u64
            });
            score += row_score;
        }

        Ok(score * self.winning_number.unwrap() as u64)
    }

    fn matrix_transpose(&mut self) -> Vec<Vec<(u8, bool)>> {
        let mut trans = vec![Vec::with_capacity(self.data.len()); self.data[0].len()];
        for r in self.data.iter() {
            for i in 0..r.len() {
                trans[i].push(r[i]);
            }
        }
        trans
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
            chunk
                .iter()
                .map(|c| {
                    c.split(' ')
                        .into_iter()
                        .map(|value| value.trim())
                        .filter(|x| !x.is_empty())
                        .map(|p| (p.parse::<u8>().unwrap(), false))
                        .collect::<Vec<(u8, bool)>>()
                })
                .collect::<Vec<Vec<(u8, bool)>>>()
        })
        .map(|parsed_chunk| Bingoboard {
            data: parsed_chunk,
            winning_number: None,
        })
        .collect();
    (game_numbers, bingoboards)
}

fn part_one(game_numbers: &Vec<u8>, boards: &mut Vec<Bingoboard>) -> Result<u64, String> {
    for (idx, num) in game_numbers.into_iter().enumerate() {
        for board in boards.iter_mut() {
            board.change_state_by_number(*num);
            if idx >= 5 {
                // bingo can only occure if we already had 5 numbers
                if board.check_bingo() {
                    board.winning_number = Some(*num);
                    return Ok(board.score()?);
                }
            }
        }
    }
    Err(String::from("Something went wrong!"))
}

fn part_two(game_numbers: &Vec<u8>, boards: &mut Vec<Bingoboard>) -> Result<u64, String> {
    let mut bingos: VecDeque<Bingoboard> = VecDeque::new();
    for (idx, num) in game_numbers.into_iter().enumerate() {
        for board in boards.iter_mut() {
            board.change_state_by_number(*num);
            if idx >= 5 {
                // bingo can only occure if we already had 5 numbers
                if board.check_bingo() && board.winning_number.is_none() {
                    board.winning_number = Some(*num);
                    bingos.push_back(board.clone());
                }
            }
        }
    }
    if !bingos.is_empty() {
        return Ok(bingos.pop_back().unwrap().clone().score()?);
    }
    Err(String::from("Did not found any winning board!"))
}

fn main() {
    let (game_numbers, mut bingoboards) = parse_input(include_str!("../input.txt"));

    println!("Part 1:");
    let score = part_one(&game_numbers, &mut bingoboards.clone());
    println!("Score of the winner board:\n{}", score.unwrap());

    println!("Part 2:");
    let score_two = part_two(&game_numbers, &mut bingoboards);
    println!("Score of the last winner board:\n{}", score_two.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let mut input =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n"
                .to_string();

        let board1 = &[
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
        .join("\n");

        let board2 = &[
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
        ]
        .join("\n");

        let board3 = &[
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .join("\n");
        input.push_str(board1);
        input.push('\n');
        input.push_str(board2);
        input.push('\n');
        input.push_str(board3);
        let (game_numbers, bingoboards) = parse_input(&input);
        assert_eq!(bingoboards.len(), 3);
        assert_eq!(game_numbers.len(), 27);
    }

    #[test]
    fn test_part_one() {
        let mut input =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n"
                .to_string();

        let board1 = &[
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
        .join("\n");

        let board2 = &[
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
        ]
        .join("\n");

        let board3 = &[
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .join("\n");
        input.push_str(board1);
        input.push('\n');
        input.push_str(board2);
        input.push('\n');
        input.push_str(board3);
        let (game_numbers, mut bingoboards) = parse_input(&input);
        let score = part_one(&game_numbers, &mut bingoboards);
        assert_eq!(score.unwrap(), 4512);
    }

    #[test]
    fn test_part_two() {
        let mut input =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n"
                .to_string();

        let board1 = &[
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
        .join("\n");

        let board2 = &[
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
        ]
        .join("\n");

        let board3 = &[
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .join("\n");
        input.push_str(board1);
        input.push('\n');
        input.push_str(board2);
        input.push('\n');
        input.push_str(board3);
        let (game_numbers, mut bingoboards) = parse_input(&input);
        let score = part_two(&game_numbers, &mut bingoboards);
        assert_eq!(score.unwrap(), 1924);
    }
}
