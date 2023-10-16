use action_chess::state::{
    gamestate::GameState,
    piece::{Color, Move},
    square::*,
};

fn main() {
    let mut state: GameState = GameState::new();

    println!("{}", state.board);
}
