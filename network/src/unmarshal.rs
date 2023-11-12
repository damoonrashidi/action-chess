use crate::{
    command::Command,
    constants::{
        COLOR_BLACK, GAME_JOIN, GAME_LEAVE, GAME_RESIGN, PIECE_BISHOP, PIECE_KING, PIECE_KNIGHT,
        PIECE_PAWN, PIECE_QUEEN, PIECE_ROOK,
    },
    game_command::GameCmd,
};
use state::{
    cooldowns::{
        COOLDOWN_BISHOP, COOLDOWN_KING, COOLDOWN_KNIGHT, COOLDOWN_PAWN, COOLDOWN_QUEEN,
        COOLDOWN_ROOK,
    },
    coordinate::Coord,
    piece::{Color, Piece},
};

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        if value & COLOR_BLACK == COLOR_BLACK {
            Color::Black
        } else {
            Color::White
        }
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

impl From<u8> for Piece {
    fn from(value: u8) -> Self {
        let color = (value & 1).into();

        let v = value >> 4;
        let pieces = [
            (PIECE_PAWN, Piece::Pawn(color, COOLDOWN_PAWN)),
            (PIECE_KNIGHT, Piece::Knight(color, COOLDOWN_KNIGHT)),
            (PIECE_BISHOP, Piece::Bishop(color, COOLDOWN_BISHOP)),
            (PIECE_ROOK, Piece::Rook(color, COOLDOWN_ROOK)),
            (PIECE_QUEEN, Piece::Queen(color, COOLDOWN_QUEEN)),
            (PIECE_KING, Piece::King(color, COOLDOWN_KING)),
        ];

        for (comp, piece) in pieces {
            if comp >> 4 == v {
                return piece;
            }
        }

        panic!("could not decode {value}");
    }
}

impl From<Command> for GameCmd {
    fn from(value: Command) -> Self {
        match value {
            [GAME_JOIN, p1, p2, p3] => GameCmd::Join(String::from_utf8(vec![p1, p2, p3]).unwrap()),
            [GAME_LEAVE, ..] => GameCmd::Leave,
            [GAME_RESIGN, ..] => GameCmd::Resign,
            [cmd, ..] => panic!("invalid lead byte {cmd}"),
        }
    }
}

impl From<Command> for Move {
    fn from(value: Command) -> Self {
        match value {
            [MOVE_PIECE, ..] => decode_move(&value),
            [MOVE_PROMOTION, ..] => decode_promotion(&value),
            [MOVE_KING_SIDE_CASTLE, ..] => decode_ksc(&value),
            [MOVE_QUEEN_SIDE_CASTLE, ..] => decode_qsc(&value),
            [cmd, ..] => panic!("invalid lead byte {cmd}"),
        }
    }
}

fn decode_move(bytes: &[u8]) -> Move {
    let from = bytes[1];
    let to = bytes[2];

    Move::Piece(from.into(), to.into())
}

fn decode_promotion(bytes: &[u8]) -> Move {
    let from = bytes[1];
    let to = bytes[2];
    let piece = bytes[3];

    Move::Promotion(from.into(), to.into(), piece.into())
}

fn decode_ksc(bytes: &[u8]) -> Move {
    Move::KingSideCastle(bytes[1].into())
}

fn decode_qsc(bytes: &[u8]) -> Move {
    Move::QueenSideCastle(bytes[1].into())
}
