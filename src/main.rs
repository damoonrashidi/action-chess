use action_chess::state::{
    gamestate::GameState,
    piece::{Color, Move},
    square::*,
};

fn main() {
    let mut state: GameState = GameState::new();

    let moves = [
        Move::Piece(E2, E4),
        Move::Piece(D2, D4),
        Move::Piece(G1, F3),
        Move::Piece(F1, C4),
        Move::KingSideCastle(Color::White),
    ];

    for m in moves {
        state.board.process_move(m);
    }

    println!("{}", state.board);
}
