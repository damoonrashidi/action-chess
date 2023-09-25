use super::board::Board;

#[derive(Clone)]
pub struct GameState {
    pub board: Board,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: { Board::new() },
        }
    }
}
