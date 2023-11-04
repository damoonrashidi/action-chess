#[cfg(test)]
mod test {
    use crate::{
        network::command::Command,
        state::{
            cooldowns::{COOLDOWN_QUEEN, COOLDOWN_ROOK},
            coordinate::Coord,
            piece::{Color, Move, Piece},
            square::*,
        },
    };

    #[test]
    fn coord_into_u8() {
        for (coord, expected) in [(A1, 0), (A2, 8), (H8, 63)] {
            let actual: u8 = coord.into();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn u8_into_coord() {
        for (expected, index) in [(A1, 0), (A2, 8), (H8, 63)] {
            let actual: Coord = index.into();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_move() {
        let mv = Move::Piece(A5, C5);
        let command: Command = mv.into();
        assert_eq!(command, Command([0, 32, 34, 0]));
    }

    #[test]
    fn serialize_promotion_white_rook() {
        let mv = Move::Promotion(D7, D8, Piece::Rook(Color::White, COOLDOWN_ROOK));
        let command: Command = mv.into();

        assert_eq!(command, Command([1, 51, 59, 48]));
    }

    #[test]
    fn serialize_promotion_black_queen() {
        let mv = Move::Promotion(A2, A1, Piece::Queen(Color::Black, COOLDOWN_QUEEN));
        let command: Command = mv.into();

        assert_eq!(command, Command([1, 8, 0, 65]));
    }

    #[test]
    fn serialize_king_side_castle_for_black() {
        let mv = Move::KingSideCastle(Color::Black);
        let command: Command = mv.into();

        assert_eq!(command, Command([2, 1, 0, 0]));
    }

    #[test]
    fn serialize_queen_side_castle_for_white() {
        let mv = Move::QueenSideCastle(Color::White);
        let command: Command = mv.into();

        assert_eq!(command, Command([3, 0, 0, 0]));
    }
}
