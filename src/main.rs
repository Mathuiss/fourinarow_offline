mod controllers;
mod models;
mod views;

use models::game_core::{Game, Player};
use views::console_view;

fn main() {
    let mut game = Game::new();

    while !&game.is_won {
        game.play_turn();
    }

    let winner = match game.current_player {
        Player::Red => Player::Yellow,
        Player::Yellow => Player::Red,
    };

    console_view::print_msg_ln(format!("Player {:?} won!", winner).as_str());
    console_view::print_msg("Exit 0");
}
