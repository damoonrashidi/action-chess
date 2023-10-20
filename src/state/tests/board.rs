#[cfg(test)]
mod tests {
    use crate::state::{
        board::Board,
        piece::{Color::Black, Piece},
    };

    #[test]
    fn get_by_color() {
        let board = Board::new();
        let black_pieces = board.get_pieces_by_color(Black);
        assert_eq!(black_pieces.len(), 16);
    }

    #[test]
    fn get_all_black_pawns() {
        let board = Board::new();
        let black_pawns = board.filter_board(Piece::Pawn(Black));
        assert_eq!(black_pawns.pieces.len(), 8);
    }
}
