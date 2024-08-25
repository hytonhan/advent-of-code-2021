advent_of_code::solution!(4);

struct BingoBoard {
    board: Vec<(u32, bool)>,
}

impl BingoBoard {
    pub fn new(board: Vec<(u32, bool)>) -> BingoBoard {
        BingoBoard { board: board }
    }

    pub fn is_winner(&self) -> bool {
        for row in 1..=5 {
            let mut row_wins = true;
            for index in (row * 5 - 5)..(row * 5 - 1) {
                row_wins &= self.board[index].1;
            }
            if row_wins {
                return true;
            }
        }
        for column in 0..5 {
            let mut column_wins = true;
            for index in (column..25).step_by(5) {
                column_wins &= self.board[index].1;
            }
            if column_wins {
                return true;
            }
        }

        false
    }

    pub fn is_row_or_column_winner(&self, index: usize) -> bool {
        let modulus = index % 5;
        let mut is_row_winner = true;
        for i in (modulus..25).step_by(5) {
            is_row_winner &= self.board[i].1
        }
        if is_row_winner {
            return is_row_winner;
        }
        let mut is_column_winner = true;
        for i in (index - modulus)..(index - modulus + 5) {
            is_column_winner &= self.board[i].1
        }

        is_column_winner
    }

    pub fn mark_number_as_found(&mut self, number: u32) -> i32 {
        for i in 0..25 {
            if self.board[i].0 == number {
                self.board[i].1 = true;
                return i as i32;
            }
        }
        -1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let draws: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|item| item.parse::<u32>().unwrap())
        .collect();

    let mut bingo_boards: Vec<BingoBoard> = input
        .split("\n\n")
        .skip(1)
        .map(|item| {
            let board_numbers = item
                .replace("\n", " ")
                .replace("  ", " ")
                .trim()
                .split(" ")
                .map(|number| (number.parse::<u32>().unwrap(), false))
                .collect();
            BingoBoard::new(board_numbers)
        })
        .collect();

    let mut winning_number = 0;
    let mut winning_board = 0;
    for x in 0..draws.len() {
        let number = draws[x];
        let mut winner_found = false;
        for i in 0..bingo_boards.len() {
            let board_index = i;
            let number_index = bingo_boards[i].mark_number_as_found(number);
            if number_index == -1 {
                continue;
            }
            if bingo_boards[board_index].is_row_or_column_winner(number_index as usize) {
                winning_number = bingo_boards[board_index].board[number_index as usize].0;
                winning_board = board_index;
                winner_found = true;
                break;
            }
        }
        if winner_found {
            break;
        }
    }

    let sum_of_unmarked = (&bingo_boards[winning_board].board)
        .into_iter()
        .filter(|item| item.1 == false)
        .map(|item| item.0)
        .sum::<u32>();
    let result = sum_of_unmarked * winning_number;
    Some(result)

    // 10384 too low
}

pub fn part_two(input: &str) -> Option<u32> {
    let draws: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|item| item.parse::<u32>().unwrap())
        .collect();

    let mut bingo_boards: Vec<BingoBoard> = input
        .split("\n\n")
        .skip(1)
        .map(|item| {
            let board_numbers = item
                .replace("\n", " ")
                .replace("  ", " ")
                .trim()
                .split(" ")
                .map(|number| (number.parse::<u32>().unwrap(), false))
                .collect();
            BingoBoard::new(board_numbers)
        })
        .collect();

    let mut winning_number = 0;
    let mut winning_board = 0;
    for x in 0..draws.len() {
        let number = draws[x];
        let mut winner_found = false;
        let mut winners: Vec<usize> = Vec::new();
        for i in 0..bingo_boards.len() {
            winner_found = false;
            let board_index = i;
            let number_index = bingo_boards[i].mark_number_as_found(number);
            if number_index == -1 {
                continue;
            }
            if bingo_boards[board_index].is_row_or_column_winner(number_index as usize) {
                winning_number = bingo_boards[board_index].board[number_index as usize].0;
                winning_board = board_index;
                winner_found = true;
                if bingo_boards.len() > 1 {
                    winners.push(winning_board);
                }
                if bingo_boards.len() == 1 {
                    break;
                }
            }
        }
        if bingo_boards.len() == 1 && winner_found {
            break;
        }
        if bingo_boards.len() > 1 {
            winners.into_iter().rev().for_each(|item| {
                bingo_boards.remove(item);
            });
        }
    }

    let sum_of_unmarked = (&bingo_boards[winning_board].board)
        .into_iter()
        .filter(|item| item.1 == false)
        .map(|item| item.0)
        .sum::<u32>();
    let result = sum_of_unmarked * winning_number;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4512));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1924));
    }
}
