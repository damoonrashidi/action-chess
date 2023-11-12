#[cfg(test)]
mod get_piece_information {
    use state::piece::{Color, Piece};

    use crate::unmarshal::Unmarshal;

    #[test]
    fn get_color_for_piece() {
        let color: Color = Unmarshal::color(1);

        assert_eq!(color, Color::Black);
    }

    #[test]
    fn get_piece() {
        let piece = Unmarshal::piece(0b0101_0000);
        assert!(matches!(piece, Piece::King(_, _)));
    }

    #[test]
    fn get_piece_with_color() {
        let black_king = Unmarshal::piece(0b0101_0001);
        let white_bishop = Unmarshal::piece(0b0010_0000);
        assert!(matches!(black_king, Piece::King(Color::Black, _)));
        assert!(matches!(white_bishop, Piece::Bishop(Color::White, _)));
    }
}

#[cfg(test)]
mod moves {

    use state::{
        cooldowns::*,
        piece::{Color, Move, Piece},
        square::*,
    };

    use crate::unmarshal::Unmarshal;

    #[test]
    fn move_piece() {
        let mv: Move = Unmarshal::command([0u8, 0, 5, 0]);
        assert_eq!(mv, Move::Piece(A1, F1));
    }

    #[test]
    fn get_promote() {
        let mv: Move = Unmarshal::command([1, 8, 0, 0b0001_0001]);
        assert_eq!(
            mv,
            Move::Promotion(A2, A1, Piece::Knight(Color::Black, COOLDOWN_BISHOP))
        );
    }

    #[test]
    fn castle_king_side() {
        let mv: Move = Unmarshal::command([2u8, 0, 0, 0]);
        assert_eq!(mv, Move::KingSideCastle(Color::White));
    }

    #[test]
    fn castle_queen_side() {
        let mv: Move = Unmarshal::command([3u8, 1, 0, 0]);
        assert_eq!(mv, Move::QueenSideCastle(Color::Black));
    }
}
