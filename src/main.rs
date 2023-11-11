pub mod server;

use std::time::Duration;

use action_chess::state::board::Board;

fn main() {
    let mut board = Board::from_fen("qrb5/rk1p1K2/p2P4/Pp6/1N2n3/6p1/5nB1/6b1 w - - 0 1").unwrap();
    for y in 0..8 {
        for x in 0..8 {
            if let Some(mut piece) = board.pieces[x][y] {
                piece.set_cooldown(Duration::ZERO);
                board.pieces[x][y] = Some(piece);
            }
        }
    }
    println!("{board}");
}
