use crate::state::{
    coordinate::Coord,
    piece::{Color, Move, Piece},
};

// A command is 4 bytes.
// first byte is reserved for the move
//      if move is Piece (0)
//          the second byte the from coord represented as as a value 0..64 -> translated to (file, rank)
//          the third byte is the to coord represented as as a value 0..64 -> translated to (file, rank)
//          the fourth byte is empty buffer byte
//      if move is Promotion (1)
//          the second byte is the from coord
//          the third byte is the to coord
//          the fourth byte is the from promotion piece -> represented
//      if move is king_side_castle (10)
//          the second byte is color
//          the third byte is empty buffer
//          the fourth byte is empty buffer
//      if move is queen_side_castle (11)
//          the second byte is color
//          the third byte is empty buffer
//          the fourth byte is empty buffer

#[derive(Debug, PartialEq)]
pub struct Command(pub [u8; 4]);

const EMPTY_BUFFER: u8 = 0b0000_0000;

const COLOR_WHITE: u8 = 0b0000_0000;
const COLOR_BLACK: u8 = 0b0000_0001;

const PIECE_PAWN: u8 = 0b0000_0000;
const PIECE_KNIGHT: u8 = 0b0001_0000;
const PIECE_BISHOP: u8 = 0b0010_0000;
const PIECE_ROOK: u8 = 0b0011_0000;
const PIECE_QUEEN: u8 = 0b0100_0000;
const PIECE_KING: u8 = 0b0101_0000;

const MOVE_PIECE: u8 = 0b0000_0000;
const MOVE_PROMOTION: u8 = 0b0000_0001;
const MOVE_KING_SIDE_CASTLE: u8 = 0b0000_0010;
const MOVE_QUEEN_SIDE_CASTLE: u8 = 0b0000_0011;

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

impl From<u8> for Coord {
    fn from(value: u8) -> Self {
        let file = value % 8;
        let rank = value / 8;
        #[allow(clippy::cast_possible_wrap)]
        Coord(file as i8, rank as i8)
    }
}

impl From<Move> for Command {
    fn from(value: Move) -> Self {
        match value {
            Move::Piece(from, to) => Command([MOVE_PIECE, from.into(), to.into(), EMPTY_BUFFER]),
            Move::Promotion(from, to, piece) => {
                Command([MOVE_PROMOTION, from.into(), to.into(), piece.into()])
            }
            Move::KingSideCastle(color) => Command([
                MOVE_KING_SIDE_CASTLE,
                color.into(),
                EMPTY_BUFFER,
                EMPTY_BUFFER,
            ]),
            Move::QueenSideCastle(color) => Command([
                MOVE_QUEEN_SIDE_CASTLE,
                color.into(),
                EMPTY_BUFFER,
                EMPTY_BUFFER,
            ]),
        }
    }
}
