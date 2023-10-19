#[cfg(test)]
mod tests {
    use crate::state::{
        board::Board,
        movegen::MoveGen,
        piece::{
            Color::{Black, White},
            Move, Piece,
        },
        square::*,
    };

    #[test]
    fn knight_on_standard_board() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);

        let e4 = movegen.for_knight(&Piece::Knight(White), &E4);

        assert_eq!(e4.len(), 6);
    }

    #[test]
    fn rook_on_empty_board() {
        let board = Board::default();
        let movegen = MoveGen::new(&board);

        let moves = movegen.for_rook(&Piece::Rook(White), &E4);

        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn rook_on_standard_board() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_rook(&Piece::Rook(White), &A5);

        let expected_moves: Vec<Move> = [A6, A7, B5, C5, D5, E5, F5, G5, H5, A4, A3]
            .iter()
            .map(|dest| Move::Piece(A5, *dest))
            .collect();

        assert!(moves.iter().all(|e| expected_moves.contains(e)));
    }

    #[test]
    fn starting_rook() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_rook(&Piece::Rook(White), &A1);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn infiltrated_starting_rook() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_rook(&Piece::Rook(Black), &A1);
        assert_eq!(vec![Move::Piece(A1, A2), Move::Piece(A1, B1)], moves);
    }

    #[test]
    fn bishop_on_empty() {
        let board = Board::default();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_bishop(&Piece::Bishop(White), &E5);

        let expected_moves: Vec<Move> = [D4, C3, B2, A1, F6, G7, H8, F4, G3, H2, D6, C7, B8]
            .iter()
            .map(|dest| Move::Piece(E5, *dest))
            .collect();

        assert!(moves.iter().all(|e| expected_moves.contains(e)));
    }

    #[test]
    fn bishop_on_standard() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_bishop(&Piece::Bishop(White), &E5);

        let expected_moves: Vec<Move> = [D4, C3, F6, G7, F4, G3, D6, C7]
            .iter()
            .map(|dest| Move::Piece(E5, *dest))
            .collect();

        assert!(moves.iter().all(|e| expected_moves.contains(e)));
    }

    #[test]
    pub fn white_pawn_at_start() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_pawn(&Piece::Pawn(White), &A5);
        let expected_moves: Vec<Move> = [A3, A4]
            .into_iter()
            .map(|dest| Move::Piece(A2, dest))
            .collect();

        assert_eq!(moves, expected_moves);
    }
}
