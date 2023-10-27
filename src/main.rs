use action_chess::state::{gamestate::GameState, movegen::MoveGen};

fn main() {
    let state = GameState::new();
    let gen = MoveGen::new(&state.board);
    let moves = gen.get_possible_moves();
    MoveGen::render_movelist(&state.board, &moves);
}
