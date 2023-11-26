use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use state::{board::Board, piece::Move};

pub(crate) fn listen(board: &Arc<Mutex<Board>>, move_listener: Receiver<Move>) -> JoinHandle<()> {
    let board = Arc::clone(board);
    thread::spawn(move || {
        for mv in move_listener {
            if let Ok(mut board) = board.lock() {
                board.process_move(mv);
                println!("{board}");
            }
        }
    })
}
