#[cfg(test)]
mod test {
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

    use crate::{
        network::command::Command,
        state::{
            cooldowns::*,
            piece::{Color, Move, Piece},
            square::*,
        },
    };

    #[test]
    fn move_piece() {
        let mv: Move = Command([0, 0, 0, 0]).into();
        assert!(matches!(mv, Move::Piece(_, _)));
    }

    #[test]
    fn get_promote() {
        let mv: Move = Command([1, 8, 0, 0b0001_0001]).into();
        println!("{mv}");
        assert!(matches!(
            mv,
            Move::Promotion(A2, A1, Piece::Knight(Color::Black, COOLDOWN_BISHOP))
        ));
    }
}
