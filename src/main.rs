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

    // --- Vertical win column 3 ---

    #[test]
    fn test_vertical_win_col_3() {
        let mut board = Board::new();
        board.make_move(3, Cell::O).unwrap();
        board.make_move(6, Cell::O).unwrap();
        board.make_move(9, Cell::O).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::O));
    }

    // --- Cell Display trait ---

    #[test]
    fn test_cell_display_x() {
        assert_eq!(format!("{}", Cell::X), "X");
    }

    #[test]
    fn test_cell_display_o() {
        assert_eq!(format!("{}", Cell::O), "O");
    }

    #[test]
    fn test_cell_display_empty() {
        assert_eq!(format!("{}", Cell::Empty), " ");
    }

    // --- Board Display trait ---

    #[test]
    fn test_board_display_empty() {
        let board = Board::new();
        let output = format!("{}", board);
        // Empty board should show position numbers 1-9
        for pos in 1..=9 {
            assert!(
                output.contains(&pos.to_string()),
                "Empty board should display position {}",
                pos
            );
        }
    }

    #[test]
    fn test_board_display_with_moves() {
        let mut board = Board::new();
        board.make_move(1, Cell::X).unwrap();
        board.make_move(5, Cell::O).unwrap();
        let output = format!("{}", board);
        assert!(output.contains('X'));
        assert!(output.contains('O'));
        // Unfilled positions should still show their numbers
        assert!(output.contains('2'));
        assert!(output.contains('3'));
    }

    #[test]
    fn test_board_display_has_separators() {
        let board = Board::new();
        let output = format!("{}", board);
        // Board should have row separators
        assert!(output.contains('-'));
        // Board should have column separators
        assert!(output.contains('|'));
    }

    // --- position_to_row_col ---

    #[test]
    fn test_position_to_row_col_all_positions() {
        let expected = [
            (1, (0, 0)),
            (2, (0, 1)),
            (3, (0, 2)),
            (4, (1, 0)),
            (5, (1, 1)),
            (6, (1, 2)),
            (7, (2, 0)),
            (8, (2, 1)),
            (9, (2, 2)),
        ];
        for (pos, (exp_row, exp_col)) in expected {
            let (row, col) = position_to_row_col(pos);
            assert_eq!(
                (row, col),
                (exp_row, exp_col),
                "Position {} should map to ({}, {})",
                pos,
                exp_row,
                exp_col
            );
        }
    }

    // --- is_full edge cases ---

    #[test]
    fn test_is_full_empty_board() {
        let board = Board::new();
        assert!(!board.is_full());
    }

    #[test]
    fn test_is_full_one_move() {
        let mut board = Board::new();
        board.make_move(5, Cell::X).unwrap();
        assert!(!board.is_full());
    }

    #[test]
    fn test_is_full_eight_moves() {
        let mut board = Board::new();
        board.make_move(1, Cell::X).unwrap();
        board.make_move(2, Cell::O).unwrap();
        board.make_move(3, Cell::X).unwrap();
        board.make_move(4, Cell::O).unwrap();
        board.make_move(5, Cell::X).unwrap();
        board.make_move(6, Cell::O).unwrap();
        board.make_move(7, Cell::X).unwrap();
        board.make_move(8, Cell::O).unwrap();
        assert!(!board.is_full());
    }

    // --- Making moves at all 9 positions ---

    #[test]
    fn test_make_move_all_positions() {
        for pos in 1..=9 {
            let mut board = Board::new();
            assert!(
                board.make_move(pos, Cell::X).is_ok(),
                "Should be able to place at position {}",
                pos
            );
            let (row, col) = position_to_row_col(pos);
            assert_eq!(board.cells[row][col], Cell::X);
        }
    }

    // --- Alternating player game simulation ---

    #[test]
    fn test_full_game_x_wins() {
        // Simulate: X=1, O=4, X=2, O=5, X=3 -> X wins row 1
        let mut board = Board::new();
        let moves = [
            (1, Cell::X),
            (4, Cell::O),
            (2, Cell::X),
            (5, Cell::O),
            (3, Cell::X),
        ];
        for (pos, player) in moves {
            assert!(board.make_move(pos, player).is_ok());
        }
        assert_eq!(board.check_winner(), Some(Cell::X));
    }

    #[test]
    fn test_full_game_o_wins() {
        // Simulate: X=1, O=3, X=4, O=5, X=8, O=7 -> O wins diagonal 3-5-7
        let mut board = Board::new();
        let moves = [
            (1, Cell::X),
            (3, Cell::O),
            (4, Cell::X),
            (5, Cell::O),
            (8, Cell::X),
            (7, Cell::O),
        ];
        for (pos, player) in moves {
            assert!(board.make_move(pos, player).is_ok());
        }
        assert_eq!(board.check_winner(), Some(Cell::O));
    }

    #[test]
    fn test_full_game_draw() {
        // X | O | X
        // O | X | X
        // O | X | O
        let mut board = Board::new();
        let moves = [
            (1, Cell::X),
            (2, Cell::O),
            (3, Cell::X),
            (4, Cell::O),
            (5, Cell::X),
            (6, Cell::X),
            (7, Cell::O),
            (8, Cell::X),
            (9, Cell::O),
        ];
        for (pos, player) in moves {
            assert!(board.make_move(pos, player).is_ok());
        }
        assert!(board.is_full());
        assert_eq!(board.check_winner(), None);
    }

    // --- Win detected immediately (no winner before the winning move) ---

    #[test]
    fn test_no_winner_before_winning_move() {
        let mut board = Board::new();
        board.make_move(1, Cell::X).unwrap();
        board.make_move(2, Cell::X).unwrap();
        assert_eq!(board.check_winner(), None);
        board.make_move(3, Cell::X).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::X));
    }

    // --- O winning scenarios ---

    #[test]
    fn test_o_wins_horizontal_row_1() {
        let mut board = Board::new();
        board.make_move(1, Cell::O).unwrap();
        board.make_move(2, Cell::O).unwrap();
        board.make_move(3, Cell::O).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::O));
    }

    #[test]
    fn test_o_wins_horizontal_row_2() {
        let mut board = Board::new();
        board.make_move(4, Cell::O).unwrap();
        board.make_move(5, Cell::O).unwrap();
        board.make_move(6, Cell::O).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::O));
    }

    #[test]
    fn test_o_wins_horizontal_row_3() {
        let mut board = Board::new();
        board.make_move(7, Cell::O).unwrap();
        board.make_move(8, Cell::O).unwrap();
        board.make_move(9, Cell::O).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::O));
    }

    #[test]
    fn test_o_wins_vertical_col_1() {
        let mut board = Board::new();
        board.make_move(1, Cell::O).unwrap();
        board.make_move(4, Cell::O).unwrap();
        board.make_move(7, Cell::O).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::O));
    }

    #[test]
    fn test_o_wins_vertical_col_2() {
        let mut board = Board::new();
        board.make_move(2, Cell::O).unwrap();
        board.make_move(5, Cell::O).unwrap();
        board.make_move(8, Cell::O).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::O));
    }

    #[test]
    fn test_o_wins_diagonal_top_left_to_bottom_right() {
        let mut board = Board::new();
        board.make_move(1, Cell::O).unwrap();
        board.make_move(5, Cell::O).unwrap();
        board.make_move(9, Cell::O).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::O));
    }

    // --- Error message content validation ---

    #[test]
    fn test_error_message_invalid_position_zero_content() {
        let mut board = Board::new();
        let err = board.make_move(0, Cell::X).unwrap_err();
        assert!(err.contains("Invalid position: 0"));
        assert!(err.contains("between 1 and 9"));
    }

    #[test]
    fn test_error_message_invalid_position_ten_content() {
        let mut board = Board::new();
        let err = board.make_move(10, Cell::X).unwrap_err();
        assert!(err.contains("Invalid position: 10"));
        assert!(err.contains("between 1 and 9"));
    }

    #[test]
    fn test_error_message_occupied_cell_content() {
        let mut board = Board::new();
        board.make_move(5, Cell::X).unwrap();
        let err = board.make_move(5, Cell::O).unwrap_err();
        assert!(err.contains("Position 5"));
        assert!(err.contains("already occupied"));
    }

    // --- Large invalid positions ---

    #[test]
    fn test_invalid_position_100() {
        let mut board = Board::new();
        let result = board.make_move(100, Cell::X);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid position: 100"));
    }

    #[test]
    fn test_invalid_position_usize_max() {
        let mut board = Board::new();
        let result = board.make_move(usize::MAX, Cell::X);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid position"));
    }

    // --- Additional edge cases ---

    #[test]
    fn test_check_winner_empty_board() {
        let board = Board::new();
        assert_eq!(board.check_winner(), None);
    }

    #[test]
    fn test_same_player_cannot_overwrite_own_cell() {
        let mut board = Board::new();
        board.make_move(1, Cell::X).unwrap();
        let result = board.make_move(1, Cell::X);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already occupied"));
    }

    #[test]
    fn test_cell_equality() {
        assert_eq!(Cell::X, Cell::X);
        assert_eq!(Cell::O, Cell::O);
        assert_eq!(Cell::Empty, Cell::Empty);
        assert_ne!(Cell::X, Cell::O);
        assert_ne!(Cell::X, Cell::Empty);
        assert_ne!(Cell::O, Cell::Empty);
    }

    #[test]
    fn test_cell_debug() {
        assert_eq!(format!("{:?}", Cell::X), "X");
        assert_eq!(format!("{:?}", Cell::O), "O");
        assert_eq!(format!("{:?}", Cell::Empty), "Empty");
    }

    #[test]
    fn test_cell_clone_and_copy() {
        let cell = Cell::X;
        let cloned = cell.clone();
        let copied = cell;
        assert_eq!(cell, cloned);
        assert_eq!(cell, copied);
    }

    #[test]
    fn test_vertical_win_col_3_x() {
        let mut board = Board::new();
        board.make_move(3, Cell::X).unwrap();
        board.make_move(6, Cell::X).unwrap();
        board.make_move(9, Cell::X).unwrap();
        assert_eq!(board.check_winner(), Some(Cell::X));
    }

    #[test]
    fn test_board_display_full_board() {
        let mut board = Board::new();
        let moves = [
            (1, Cell::X),
            (2, Cell::O),
            (3, Cell::X),
            (4, Cell::X),
            (5, Cell::X),
            (6, Cell::O),
            (7, Cell::O),
            (8, Cell::X),
            (9, Cell::O),
        ];
        for (pos, player) in moves {
            board.make_move(pos, player).unwrap();
        }
        let output = format!("{}", board);
        assert!(output.contains('X'));
        assert!(output.contains('O'));
        // Full board should not display any position digits
        for digit in ['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
            assert!(
                !output.contains(digit),
                "Full board should not display position digit '{}'",
                digit
            );
        }
    }

    #[test]
    fn test_no_winner_mixed_rows_cols() {
        // Board state where rows and cols are mixed (no winner)
        // X | O | X
        // O | X | O
        // _ | _ | _
        let mut board = Board::new();
        board.make_move(1, Cell::X).unwrap();
        board.make_move(2, Cell::O).unwrap();
        board.make_move(3, Cell::X).unwrap();
        board.make_move(4, Cell::O).unwrap();
        board.make_move(5, Cell::X).unwrap();
        board.make_move(6, Cell::O).unwrap();
        assert_eq!(board.check_winner(), None);
        assert!(!board.is_full());
    }
}
