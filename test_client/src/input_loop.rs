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
            let mut input = String::new();
            stdin().read_line(&mut input)?;

            if input.trim() == "debug" {
                if let Ok(board) = board.lock() {
                    let (attacks_on_white, attacks_on_black) = board.king_check_count();
                    println!("Attacks on White: {attacks_on_white}");
                    println!("Attacks on Black: {attacks_on_black}");
                    println!("White HP: {}", board.white_hp);
                    println!("Black HP: {}", board.black_hp);
                }
            } else if let Some(mv) = parse_move(&input) {
                client.make_move(mv);
            } else if !input.trim().is_empty() {
                println!(
                    "could not parse {} into move, board state unchanged",
                    input.trim()
                );
            }
        }
    })
}
