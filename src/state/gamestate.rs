use super::board::Board;

#[derive(Debug, Clone)]
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

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
