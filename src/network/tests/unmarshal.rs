#[cfg(test)]
mod get_piece_information {
    use crate::state::piece::{Color, Piece};

    #[test]
    fn get_color_for_piece() {
        let color: Color = 1.into();

        assert_eq!(color, Color::Black);
    }

    #[test]
    fn get_piece() {
        let piece = 0b0101_0000.into();
        assert!(matches!(piece, Piece::King(_, _)));
    }

    #[test]
    fn get_piece_with_color() {
        let black_king = 0b0101_0001.into();
        let white_bishop = 0b0010_0000.into();
        assert!(matches!(black_king, Piece::King(Color::Black, _)));
        assert!(matches!(white_bishop, Piece::Bishop(Color::White, _)));
    }
}

#[cfg(test)]
mod moves {

    use crate::state::{
        cooldowns::*,
        piece::{Color, Move, Piece},
        square::*,
    };

    #[test]
    fn move_piece() {
        let mv: Move = [0, 0, 5, 0].into();
        assert_eq!(mv, Move::Piece(A1, F1));
    }

    #[test]
    fn get_promote() {
        let mv: Move = [1, 8, 0, 0b0001_0001].into();
        assert_eq!(
            mv,
            Move::Promotion(A2, A1, Piece::Knight(Color::Black, COOLDOWN_BISHOP))
        );
    }

    #[test]
    fn castle_king_side() {
        let mv: Move = [2, 0, 0, 0].into();
        assert_eq!(mv, Move::KingSideCastle(Color::White));
    }

    #[test]
    fn castle_queen_side() {
        let mv: Move = [3, 1, 0, 0].into();
        assert_eq!(mv, Move::QueenSideCastle(Color::Black));
    }
}
