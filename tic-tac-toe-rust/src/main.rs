
use std::fmt::{self, Display};
use std::io::{self, Write};

// Enum to represent the state of a single cell on the board.
// Using derive traits for easy comparison, cloning, and debugging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    X,
    O,
}

// Implement the Display trait to define how a Cell should be printed.
// This is more idiomatic than a match statement inside every print call.
impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
        }
    }
}

// Enum to represent the current player.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    X,
    O,
}

// Implement Display for Player to easily print whose turn it is.
impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

// Enum to represent the state of the game.
enum GameState {
    InProgress,
    Win(Player),
    Draw,
}

/// The main entry point of the application.
fn main() {
    // Initialize the 3x3 game board with empty cells.
    let mut board = [[Cell::Empty; 3]; 3];
    let mut current_player = Player::X;

    println!("Welcome to Tic-Tac-Toe in Rust!");

    // The main game loop. It continues until a win or a draw occurs.
    loop {
        // Clear the screen for a cleaner presentation on each turn.
        // This uses ANSI escape codes.
        print!("\x1B[2J\x1B[1;1H");

        println!("Player {}'s turn.", current_player);
        print_board(&board);

        let (row, col) = get_player_move(&board, current_player);

        // Update the board with the player's move.
        board[row][col] = match current_player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        };

        // Check the state of the game after the move.
        match check_game_state(&board) {
            GameState::Win(winner) => {
                print!("\x1B[2J\x1B[1;1H");
                println!("Congratulations, Player {} wins!", winner);
                print_board(&board);
                break; // Exit the game loop.
            }
            GameState::Draw => {
                print!("\x1B[2J\x1B[1;1H");
                println!("The game is a draw!");
                print_board(&board);
                break; // Exit the game loop.
            }
            GameState::InProgress => {
                // The game continues, switch to the other player.
                current_player = match current_player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                };
            }
        }
    }
}

/// Prints the current state of the game board to the console.
fn print_board(board: &[[Cell; 3]; 3]) {
    println!("\n   0   1   2");
    println!("  -----------");
    for (i, row) in board.iter().enumerate() {
        println!("{}| {} | {} | {} |", i, row[0], row[1], row[2]);
        println!("  -----------");
    }
    println!();
}

/// Prompts the current player for their move and validates it.
/// Loops until a valid, unoccupied cell is chosen.
fn get_player_move(board: &[[Cell; 3]; 3], player: Player) -> (usize, usize) {
    loop {
        print!("Player {}, enter your move (row col): ", player);
        // We need to flush stdout to ensure the prompt is displayed before reading input.
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let parts: Vec<Result<usize, _>> = input.trim().split_whitespace().map(|s| s.parse()).collect();

        if parts.len() == 2 {
            if let (Ok(row), Ok(col)) = (&parts[0], &parts[1]) {
                if *row < 3 && *col < 3 {
                    if board[*row][*col] == Cell::Empty {
                        return (*row, *col); // Valid move
                    } else {
                        println!("This cell is already taken! Try again.");
                    }
                } else {
                    println!("Invalid input. Row and column must be between 0 and 2.");
                }
            } else {
                println!("Invalid input. Please enter two numbers separated by a space.");
            }
        } else {
            println!("Invalid input. Please enter exactly two numbers (row and column).");
        }
    }
}

/// Checks the board for a win, draw, or in-progress state.
fn check_game_state(board: &[[Cell; 3]; 3]) -> GameState {
    // Check rows and columns for a win
    for i in 0..3 {
        // Check row i
        if board[i][0] != Cell::Empty && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return GameState::Win(if board[i][0] == Cell::X { Player::X } else { Player::O });
        }
        // Check column i
        if board[0][i] != Cell::Empty && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return GameState::Win(if board[0][i] == Cell::X { Player::X } else { Player::O });
        }
    }

    // Check diagonals for a win
    if board[0][0] != Cell::Empty && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return GameState::Win(if board[0][0] == Cell::X { Player::X } else { Player::O });
    }
    if board[0][2] != Cell::Empty && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return GameState::Win(if board[0][2] == Cell::X { Player::X } else { Player::O });
    }

    // Check for a draw (if no winner and the board is full)
    if board.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty)) {
        return GameState::Draw;
    }

    // If no win or draw, the game is still in progress
    GameState::InProgress
}


