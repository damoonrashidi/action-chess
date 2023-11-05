use crate::{
    network::{
        command::Command,
        constants::{
            BUFFER_BYTE, COLOR_BLACK, COLOR_WHITE, MOVE_KING_SIDE_CASTLE, MOVE_PIECE,
            MOVE_PROMOTION, MOVE_QUEEN_SIDE_CASTLE, PIECE_BISHOP, PIECE_KING, PIECE_KNIGHT,
            PIECE_PAWN, PIECE_QUEEN, PIECE_ROOK,
        },
    },
    state::{
        coordinate::Coord,
        piece::{Color, Move, Piece},
    },
};

impl From<Piece> for u8 {
    fn from(value: Piece) -> Self {
        let color = match value.get_color() {
            Color::White => COLOR_WHITE,
            Color::Black => COLOR_BLACK,
        };
        match value {
            Piece::Pawn(_, _) => PIECE_PAWN | color,
            Piece::Knight(_, _) => PIECE_KNIGHT | color,
            Piece::Bishop(_, _) => PIECE_BISHOP | color,
            Piece::Rook(_, _) => PIECE_ROOK | color,
            Piece::Queen(_, _) => PIECE_QUEEN | color,
            Piece::King(_, _) => PIECE_KING | color,
        }
    }
}

impl From<Color> for u8 {
    fn from(value: Color) -> Self {
        match value {
            Color::White => COLOR_WHITE,
            Color::Black => COLOR_BLACK,
        }
    }
}

impl From<Coord> for u8 {
    fn from(value: Coord) -> Self {
        #[allow(clippy::cast_sign_loss)]
        let file = value.0 as u8;
        #[allow(clippy::cast_sign_loss)]
        let rank = value.1 as u8;
        rank * 8 + file
    }
}

impl From<Move> for Command {
    fn from(value: Move) -> Self {
        match value {
            Move::Piece(from, to) => Command([MOVE_PIECE, from.into(), to.into(), BUFFER_BYTE]),
            Move::Promotion(from, to, piece) => {
                Command([MOVE_PROMOTION, from.into(), to.into(), piece.into()])
            }
            Move::KingSideCastle(color) => Command([
                MOVE_KING_SIDE_CASTLE,
                color.into(),
                BUFFER_BYTE,
                BUFFER_BYTE,
            ]),
            Move::QueenSideCastle(color) => Command([
                MOVE_QUEEN_SIDE_CASTLE,
                color.into(),
                BUFFER_BYTE,
                BUFFER_BYTE,
            ]),
        }
    }
}
