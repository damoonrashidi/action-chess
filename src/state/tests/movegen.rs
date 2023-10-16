#[cfg(test)]
mod tests {
    use crate::state::{
        board::Board,
        movegen::MoveGen,
        piece::{Color::White, Piece},
        square::A1,
    };

    #[test]
    fn test_knight_movement() {
        let board = Board::default();
        let movegen = MoveGen::new(&board);

        let moves = movegen.get_moves_for_knight(&Piece::Knight(White), &A1);
        println!("{:?}", moves);

        assert_eq!(moves.len(), 2);
    }
}
