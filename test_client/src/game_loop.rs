use std::{
    sync::{Arc, Mutex},
    thread,
};

use state::{board::Board, cooldowns::BOARD_TICK_RATE};

pub(crate) fn game_loop(board: &Arc<Mutex<Board>>) -> std::thread::JoinHandle<()> {
    let board = Arc::clone(board);
    thread::spawn(move || loop {
        if let Ok(mut board) = board.lock() {
            board.tick();
        }
        thread::sleep(BOARD_TICK_RATE);
    })
}
