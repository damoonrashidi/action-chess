use crate::{
    command::Command,
    constants::{
        BUFFER_BYTE, COLOR_BLACK, COLOR_WHITE, MOVE_KING_SIDE_CASTLE, MOVE_PIECE, MOVE_PROMOTION,
        MOVE_QUEEN_SIDE_CASTLE, PIECE_BISHOP, PIECE_KING, PIECE_KNIGHT, PIECE_PAWN, PIECE_QUEEN,
        PIECE_ROOK,
    },
};
use state::{
    coordinate::Coord,
    piece::{Color, Move, Piece},
};

use super::{
    constants::{GAME_JOIN, GAME_LEAVE, GAME_RESIGN},
    game_command::GameCmd,
};

pub struct Marshal;

impl Marshal {
    pub fn color(value: Color) -> u8 {
        match value {
            Color::White => COLOR_WHITE,
            Color::Black => COLOR_BLACK,
        }
    }

    pub fn piece(value: Piece) -> u8 {
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

    pub fn coord(value: Coord) -> u8 {
        let file = value.0 as u8;
        let rank = value.1 as u8;
        rank * 8 + file
    }

    pub fn command(value: Move) -> Command {
        match value {
            Move::Piece(from, to) => [
                MOVE_PIECE,
                Marshal::coord(from),
                Marshal::coord(to),
                BUFFER_BYTE,
            ],
            Move::Promotion(from, to, piece) => [
                MOVE_PROMOTION,
                Marshal::coord(from),
                Marshal::coord(to),
                Marshal::piece(piece),
            ],
            Move::KingSideCastle(color) => [
                MOVE_KING_SIDE_CASTLE,
                Marshal::color(color),
                BUFFER_BYTE,
                BUFFER_BYTE,
            ],
            Move::QueenSideCastle(color) => [
                MOVE_QUEEN_SIDE_CASTLE,
                Marshal::color(color),
                BUFFER_BYTE,
                BUFFER_BYTE,
            ],
        }
    }

    pub fn game_command(value: GameCmd) -> Command {
        match value {
            GameCmd::Join(game_id) => {
                let game_id_bytes = game_id.as_bytes();
                [
                    GAME_JOIN,
                    game_id_bytes[0],
                    game_id_bytes[1],
                    game_id_bytes[2],
                ]
            }
            GameCmd::Leave => [GAME_LEAVE, 0, 0, 0],
            GameCmd::Resign => [GAME_RESIGN, 0, 0, 0],
        }
    }
}
