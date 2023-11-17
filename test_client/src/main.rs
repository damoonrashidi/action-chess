use std::{
    sync::{Arc, Mutex},
    thread,
};

use chess_client::ChessClient;
use state::{
    board::Board,
    piece::Move,
    square::{E2, E4},
};

fn main() -> std::io::Result<()> {
    let client = ChessClient::new("127.0.0.1:8080")?;
    let board = Arc::new(Mutex::new(Board::standard()));
    let channel = client.listen();

    client.join_game("abc");

    let b = Arc::clone(&board);
    let handle = thread::spawn(move || {
        for mv in channel {
            let mut board = b.lock().unwrap();
            board.process_move(mv);
            println!("{board}");
        }
    });

    client.make_move(Move::Piece(E2, E4));

    let _ = handle.join();

    Ok(())
}
