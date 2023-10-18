use action_chess::state::gamestate::GameState;

fn main() {
    let state: GameState = GameState::new();

    println!("{}", state.board);
}
