#[cfg(test)]
mod tests {
    use crate::state::{
        board::Board,
        movegen::MoveGen,
        piece::{Color::White, Piece},
        square::*,
    };

    #[test]
    fn test_knight_movement() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);

        let e4 = movegen.get_moves_for_knight(&Piece::Knight(White), &E4);

        assert_eq!(e4.len(), 6);
    }

    #[test]
    fn test_rook_on_empty_board() {
        let board = Board::default();
        let movegen = MoveGen::new(&board);

        let moves = movegen.get_moves_for_rook(&Piece::Rook(White), &E4);

        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn test_rook_on_standard_board() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.get_moves_for_rook(&Piece::Rook(White), &A5);

        moves.iter().for_each(|m| println!("{m}"));

        assert_eq!(moves.len(), 11);
    }
}
