use crate::models::game_core::{Coin, Player};
use std::io::{stdout, Write};

pub fn print_turn(player: &Player) {
    match player {
        Player::Red => {
            print!("RED's turn: ");
        }
        Player::Yellow => {
            print!("YELLOW's turn: ");
        }
    }

    stdout().flush().unwrap();
}

pub fn print_board(board: &[[Coin; 7]; 6]) {
    // Clear console window with control byte 27
    print!("\x1B[2J");

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            match board[y][x] {
                Coin::No => {
                    print!("[ ]");
                }
                Coin::Red => {
                    print!("[R]");
                }
                Coin::Yellow => {
                    print!("[Y]");
                }
            }
        }

        print!("\n");
    }

    stdout().flush().unwrap();
    println!("_____________________");
    println!("[1][2][3][4][5][6][7]");
    println!();
}

pub fn print_msg(msg: &str) {
    println!("{}", msg);
}
