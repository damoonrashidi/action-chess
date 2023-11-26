mod game_loop;
mod input_loop;
mod parse_input;

use crate::{game_loop::game_loop, input_loop::input_loop};
use std::{
    sync::{Arc, Mutex},
    thread::{self},
};

use chess_client::ChessClient;
use clap::Parser;
use state::board::Board;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("8000"))]
    port: String,

    #[arg(long)]
    host: String,

    #[arg(short, long)]
    game_id: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let client = Arc::new(ChessClient::new(&args.port, &args.host)?);
    let board = Arc::new(Mutex::new(Board::standard()));
    let move_listener = client.listen();

    client.join_game(&args.game_id);

    println!("Welcome to game {}", args.game_id);
    println!("{}", Board::standard());

    let remote_board = Arc::clone(&board);
    let incoming_commands = thread::spawn(move || {
        for mv in move_listener {
            if let Ok(mut board) = remote_board.lock() {
                board.process_move(mv);
                println!("{board}");
            }
        }
    });

    let game_loop_handle = game_loop(Arc::clone(&board));
    let input_handle = input_loop(client, board);

    let _ = incoming_commands.join();
    let _ = input_handle.join();
    let _ = game_loop_handle.join();

    Ok(())
}
