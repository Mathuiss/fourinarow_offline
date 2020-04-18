mod controllers;
mod models;
mod views;

use models::game_core::Game;

fn main() {
    let mut game = Game::new();

    while !&game.check_won() {
        game.play_turn();
    }
}
