use crate::controllers::console_controller;
use crate::views::console_view;

pub enum Player {
    Red,
    Yellow,
}

#[derive(Copy, Clone, Debug)]
pub enum Coin {
    Red,
    Yellow,
    No,
}

pub struct Game {
    board: [[Coin; 7]; 6],
    current_player: Player,
    is_won: bool,
}

impl Game {
    pub fn new() -> Game {
        let g = Game {
            board: [[Coin::No; 7]; 6],
            current_player: Player::Red,
            is_won: false,
        };

        console_view::print_board(&g.board);
        g
    }

    pub fn check_won(&self) -> bool {
        self.is_won
    }

    pub fn play_turn(&mut self) {
        loop {
            // Show current player
            console_view::print_turn(&self.current_player);

            // Get input
            let col = console_controller::read_col();

            // Place coin
            // -1 to adjust for 0 based array
            if self.place_coin(col - 1) {
                break;
            } else {
                console_view::print_msg_ln(
                    format!("Column {} is full. Place it somewhere else.", col).as_str(),
                );
            }
        }

        // evaluate_win
        self.evaluate_win();

        // Reset current player
        self.reset_current_player();

        // Show screen
        console_view::print_board(&self.board);
    }

    fn evaluate_win(&self) {
        println!("Evaluation later");
    }

    fn place_coin(&mut self, col: u8) -> bool {
        let mut has_placed = false;

        for y in (0..6).rev() {
            match self.board[y as usize][col as usize] {
                Coin::No => {
                    self.board[y as usize][col as usize] = match self.current_player {
                        Player::Red => Coin::Red,
                        Player::Yellow => Coin::Yellow,
                    };
                    has_placed = true;
                    break;
                }
                _ => continue,
            }
        }

        has_placed
    }

    fn reset_current_player(&mut self) {
        self.current_player = match self.current_player {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
}
