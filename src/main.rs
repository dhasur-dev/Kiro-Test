use std::fmt;
use std::io::{self, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    X,
    O,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
        }
    }
}

struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [[Cell::Empty; 3]; 3],
        }
    }

    fn make_move(&mut self, position: usize, player: Cell) -> Result<(), String> {
        if !(1..=9).contains(&position) {
            return Err(format!(
                "Invalid position: {}. Please enter a number between 1 and 9.",
                position
            ));
        }
        let (row, col) = position_to_row_col(position);
        if self.cells[row][col] != Cell::Empty {
            return Err(format!("Position {} is already occupied.", position));
        }
        self.cells[row][col] = player;
        Ok(())
    }

    fn check_winner(&self) -> Option<Cell> {
        // Check rows
        for row in 0..3 {
            if self.cells[row][0] != Cell::Empty
                && self.cells[row][0] == self.cells[row][1]
                && self.cells[row][1] == self.cells[row][2]
            {
                return Some(self.cells[row][0]);
            }
        }
        // Check columns
        for col in 0..3 {
            if self.cells[0][col] != Cell::Empty
                && self.cells[0][col] == self.cells[1][col]
                && self.cells[1][col] == self.cells[2][col]
            {
                return Some(self.cells[0][col]);
            }
        }
        // Check diagonals
        if self.cells[0][0] != Cell::Empty
            && self.cells[0][0] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][2]
        {
            return Some(self.cells[0][0]);
        }
        if self.cells[0][2] != Cell::Empty
            && self.cells[0][2] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][0]
        {
            return Some(self.cells[0][2]);
        }
        None
    }

    fn is_full(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|cell| *cell != Cell::Empty))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..3 {
            for col in 0..3 {
                let pos = row * 3 + col + 1;
                if col > 0 {
                    write!(f, " | ")?;
                } else {
                    write!(f, " ")?;
                }
                if self.cells[row][col] == Cell::Empty {
                    write!(f, "{}", pos)?;
                } else {
                    write!(f, "{}", self.cells[row][col])?;
                }
            }
            writeln!(f)?;
            if row < 2 {
                writeln!(f, "-----------")?;
            }
        }
        Ok(())
    }
}

fn position_to_row_col(position: usize) -> (usize, usize) {
    let index = position - 1;
    (index / 3, index % 3)
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn main() {
    println!("Welcome to Tic Tac Toe!");
    println!();

    loop {
        let mut board = Board::new();
        let mut current_player = Cell::X;

        loop {
            println!("{}", board);

            let player_name = if current_player == Cell::X { "X" } else { "O" };
            print!("Player {}, enter your move (1-9): ", player_name);
            io::stdout().flush().expect("Failed to flush stdout");

            let input = read_line();
            let trimmed = input.trim();

            let position: usize = match trimmed.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!(
                        "Invalid input: '{}'. Please enter a number between 1 and 9.",
                        trimmed
                    );
                    println!();
                    continue;
                }
            };

            if let Err(msg) = board.make_move(position, current_player) {
                println!("{}", msg);
                println!();
                continue;
            }

            if let Some(winner) = board.check_winner() {
                println!("{}", board);
                println!("Player {} wins!", winner);
                break;
            }

            if board.is_full() {
                println!("{}", board);
                println!("It's a draw!");
                break;
            }

            current_player = if current_player == Cell::X {
                Cell::O
            } else {
                Cell::X
            };
        }

        println!();
        print!("Play again? (y/n): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let input = read_line();
        let answer = input.trim().to_lowercase();
        if answer != "y" {
            println!("Thanks for playing!");
            break;
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(board.cells[row][col], Cell::Empty);
            }
        }
    }

    #[test]
    fn test_make_valid_move() {
        let mut board = Board::new();
        assert!(board.make_move(5, Cell::X).is_ok());
        let (row, col) = position_to_row_col(5);
        assert_eq!(board.cells[row][col], Cell::X);
    }

    #[test]
    fn test_make_move_occupied_cell() {
        let mut board = Board::new();
        board.make_move(1, Cell::X).unwrap();
        let result = board.make_move(1, Cell::O);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already occupied"));
    }

    #[test]
    fn test_make_move_invalid_position_zero() {
        let mut board = Board::new();
        let result = board.make_move(0, Cell::X);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid position"));
    }

    #[test]
    fn test_make_move_invalid_position_ten() {
        let mut board = Board::new();
        let result = board.make_move(10, Cell::X);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid position"));
    }

    #[test]
    fn test_horizontal_win_row_1() {
        let mut board = Board::new();
        board.make_move(1, Cell::X).unwrap();
        board.make_move(2, Cell::X).unwrap();
        board.make_move(3, Cell::X).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::X));
    }

    #[test]
    fn test_horizontal_win_row_2() {
        let mut board = Board::new();
        board.make_move(4, Cell::O).unwrap();
        board.make_move(5, Cell::O).unwrap();
        board.make_move(6, Cell::O).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::O));
    }

    #[test]
    fn test_horizontal_win_row_3() {
        let mut board = Board::new();
        board.make_move(7, Cell::X).unwrap();
        board.make_move(8, Cell::X).unwrap();
        board.make_move(9, Cell::X).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::X));
    }

    #[test]
    fn test_vertical_win_col_1() {
        let mut board = Board::new();
        board.make_move(1, Cell::X).unwrap();
        board.make_move(4, Cell::X).unwrap();
        board.make_move(7, Cell::X).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::X));
    }

    #[test]
    fn test_vertical_win_col_2() {
        let mut board = Board::new();
        board.make_move(2, Cell::O).unwrap();
        board.make_move(5, Cell::O).unwrap();
        board.make_move(8, Cell::O).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::O));
    }

    #[test]
    fn test_diagonal_win_top_left_to_bottom_right() {
        let mut board = Board::new();
        board.make_move(1, Cell::X).unwrap();
        board.make_move(5, Cell::X).unwrap();
        board.make_move(9, Cell::X).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::X));
    }

    #[test]
    fn test_diagonal_win_top_right_to_bottom_left() {
        let mut board = Board::new();
        board.make_move(3, Cell::O).unwrap();
        board.make_move(5, Cell::O).unwrap();
        board.make_move(7, Cell::O).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::O));
    }

    #[test]
    fn test_draw_detection() {
        let mut board = Board::new();
        // X | O | X
        // X | X | O
        // O | X | O
        board.make_move(1, Cell::X).unwrap();
        board.make_move(2, Cell::O).unwrap();
        board.make_move(3, Cell::X).unwrap();
        board.make_move(4, Cell::X).unwrap();
        board.make_move(5, Cell::X).unwrap();
        board.make_move(6, Cell::O).unwrap();
        board.make_move(7, Cell::O).unwrap();
        board.make_move(8, Cell::X).unwrap();
        board.make_move(9, Cell::O).unwrap();
        assert!(board.is_full());
        assert_eq!(board.check_winner(), None);
    }

    #[test]
    fn test_no_winner_incomplete_board() {
        let mut board = Board::new();
        board.make_move(1, Cell::X).unwrap();
        board.make_move(5, Cell::O).unwrap();
        assert_eq!(board.check_winner(), None);
        assert!(!board.is_full());
    }
}
