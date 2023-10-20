#[cfg(test)]
mod tests {
    use crate::state::{
        board::Board,
        piece::{Color::Black, Piece},
    };

    #[test]
    fn get_by_color() {
        let board = Board::new();
        let black_pieces = board.filter_by_color(Black);
        let mut count = 0;
        for y in 0..8 {
            for x in 0..8 {
                if black_pieces.pieces[y][x].is_some() {
                    count += 1;
                }
            }
        }
        assert_eq!(count, 16);
    }

    #[test]
    fn get_all_black_pawns() {
        let board = Board::new();
        let black_pawns = board.filter_by_piece(Piece::Pawn(Black));
        assert_eq!(black_pawns.pieces.len(), 8);
    }
}
