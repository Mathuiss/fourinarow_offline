use crate::controllers::console_controller;
use crate::views::console_view;

#[derive(Debug)]
pub enum Player {
    Red,
    Yellow,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Coin {
    Red,
    Yellow,
    No,
}

pub struct Game {
    board: [[Coin; 7]; 6],
    pub current_player: Player,
    pub is_won: bool,
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

    pub fn play_turn(&mut self) {
        loop {
            // Show current player
            console_view::print_turn(&self.current_player);

            // Get input
            let col = console_controller::read_col();

            // Place coin
            // -1 to adjust for 0 based array
            let (success, x, y) = self.place_coin(col - 1);

            if success {
                self.is_won = self.evaluate_coin_win(x, y);

                break;
            } else {
                console_view::print_msg_ln(
                    format!("Column {} is full. Place it somewhere else.", col).as_str(),
                );
            }
        }

        // Reset current player
        self.reset_current_player();

        // Show screen
        console_view::print_board(&self.board);
    }

    fn evaluate_coin_win(&mut self, x: usize, y: usize) -> bool {
        let target = self.board[y][x];

        // Evaluate horizontal
        let mut found_streak = false;
        let mut streak_counter = 0;

        let mut i = 0;
        while i < self.board[y].len() {
            if found_streak {
                if self.board[y][i] == target {
                    streak_counter += 1;
                } else {
                    streak_counter = 0;
                    found_streak = false;
                }
            } else {
                if self.board[y][i] == target {
                    streak_counter += 1;
                    found_streak = true;
                }
            }

            if streak_counter != 4 {
                i += 1;
            } else {
                return true;
            }
        }

        // Evaluate vertical
        found_streak = false;
        streak_counter = 0;

        i = 0;

        while i < self.board.len() {
            if found_streak {
                if self.board[i][x] == target {
                    streak_counter += 1;
                } else {
                    streak_counter = 0;
                    found_streak = false;
                }
            } else {
                if self.board[i][x] == target {
                    streak_counter += 1;
                    found_streak = true;
                }
            }

            if streak_counter != 4 {
                i += 1;
            } else {
                return true;
            }
        }

        // Evaluate positive slope
        found_streak = false;
        streak_counter = 0;

        // i = iterates x axis
        // n = n iterates y axis
        i = x;
        let mut n = y;

        while i != 0 && n < self.board.len() - 1 {
            i -= 1;
            n += 1;
        }

        while n != 0 && i < self.board[n].len() {
            if found_streak {
                if self.board[n][i] == target {
                    streak_counter += 1;
                } else {
                    streak_counter = 0;
                    found_streak = false;
                }
            } else {
                if self.board[n][i] == target {
                    streak_counter += 1;
                    found_streak = true;
                }
            }

            if streak_counter != 4 {
                i += 1;
                n -= 1;
            } else {
                return true;
            }
        }

        // Evaluate negative slope
        found_streak = false;
        streak_counter = 0;

        // i = iterates x axis
        // n = n iterates y axis
        i = x;
        n = y;

        // Set starting point
        while i != 0 && n != 0 {
            i -= 1;
            n -= 1;
        }

        while n < self.board.len() && i < self.board[n].len() {
            if found_streak {
                if self.board[n][i] == target {
                    streak_counter += 1;
                } else {
                    streak_counter = 0;
                    found_streak = false;
                }
            } else {
                if self.board[n][i] == target {
                    streak_counter += 1;
                    found_streak = true;
                }
            }

            if streak_counter != 4 {
                i += 1;
                n += 1;
            } else {
                return true;
            }
        }

        return false;
    }

    fn place_coin(&mut self, col: usize) -> (bool, usize, usize) {
        let mut has_placed = false;
        let mut row = 0;

        for y in (0..6).rev() {
            match self.board[y][col] {
                Coin::No => {
                    self.board[y][col] = match self.current_player {
                        Player::Red => Coin::Red,
                        Player::Yellow => Coin::Yellow,
                    };
                    row = y as usize;
                    has_placed = true;
                    break;
                }
                _ => continue,
            }
        }

        (has_placed, col, row)
    }

    fn reset_current_player(&mut self) {
        self.current_player = match self.current_player {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
}
