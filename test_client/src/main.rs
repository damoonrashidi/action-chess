mod commands;
mod game_loop;
mod input_loop;
mod parse_input;

#[allow(unused)]
use crate::{commands::listen, game_loop::game_loop, input_loop::input_loop};
use chess_client::ChessClient;
use clap::Parser;
use state::board::Board;
use std::sync::{Arc, Mutex};

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

    client.join_game(&args.game_id);

    println!("Welcome to game {}", args.game_id);
    println!("{}", Board::standard());

    let incoming_commands = listen(&board, client.listen());
    let game_loop_handle = game_loop(&board);
    let input_handle = input_loop(client, board);

    let _ = (
        incoming_commands.join(),
        input_handle.join(),
        game_loop_handle.join(),
    );

    Ok(())
}
