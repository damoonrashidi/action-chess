use std::{
    io::stdin,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use chess_client::ChessClient;
use state::board::Board;

use crate::parse_input::parse_move;

pub(crate) fn input_loop(
    client: Arc<ChessClient>,
    board: Arc<Mutex<Board>>,
) -> JoinHandle<anyhow::Result<()>> {
    thread::spawn(move || -> anyhow::Result<()> {
        loop {
            let mut str = String::new();
            stdin().read_line(&mut str)?;

            if str.trim() == "debug" {
                if let Ok(board) = board.lock() {
                    println!(
                        "Attacks on White: {}",
                        board.king_check_count(state::piece::Color::White)
                    );
                    println!(
                        "Attacks on Black: {}",
                        board.king_check_count(state::piece::Color::Black)
                    );
                    println!("White HP: {}", board.white_king_hp);
                    println!("Black HP: {}", board.black_king_hp);
                }
            } else if let Some(mv) = parse_move(&str) {
                client.make_move(mv);
            } else if !str.trim().is_empty() {
                println!(
                    "could not parse {} into move, board state unchanged",
                    str.trim()
                );
            }
        }
    })
}
