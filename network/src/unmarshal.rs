use crate::{
    command::Command,
    constants::{
        COLOR_BLACK, GAME_JOIN, GAME_LEAVE, GAME_RESIGN, MOVE_KING_SIDE_CASTLE, MOVE_PIECE,
        MOVE_PROMOTION, MOVE_QUEEN_SIDE_CASTLE, PIECE_BISHOP, PIECE_KING, PIECE_KNIGHT, PIECE_PAWN,
        PIECE_QUEEN, PIECE_ROOK,
    },
    game_command::GameCmd,
};
use state::{
    cooldowns::{
        COOLDOWN_BISHOP, COOLDOWN_KING, COOLDOWN_KNIGHT, COOLDOWN_PAWN, COOLDOWN_QUEEN,
        COOLDOWN_ROOK,
    },
    coordinate::Coord,
    piece::{Color, Move, Piece},
};

pub struct Unmarshal;

impl Unmarshal {
    #[must_use]
    pub fn color(value: u8) -> Color {
        if value & COLOR_BLACK == COLOR_BLACK {
            Color::Black
        } else {
            Color::White
        }
    }

    #[must_use]
    pub fn coord(value: u8) -> Coord {
        let file = value % 8;
        let rank = value / 8;
        #[allow(clippy::cast_possible_wrap)]
        Coord(file as i8, rank as i8)
    }

    /**
    # Panics
    The function will panic if the byte is not a valid piece.
     */
    #[must_use]
    pub fn piece(value: u8) -> Piece {
        let color: Color = Unmarshal::color(value & 1);

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

    /**
    # Panics
    The function will panic if the first byte in the command cannot be decoded.
     */
    #[must_use]
    pub fn command(value: Command) -> Move {
        match value[0] {
            MOVE_PIECE => Unmarshal::decode_move(&value),
            MOVE_PROMOTION => Unmarshal::decode_promotion(&value),
            MOVE_KING_SIDE_CASTLE => Unmarshal::decode_ksc(&value),
            MOVE_QUEEN_SIDE_CASTLE => Unmarshal::decode_qsc(&value),
            cmd => panic!("invalid lead byte {cmd}"),
        }
    }

    fn decode_move(bytes: &[u8]) -> Move {
        let from = bytes[1];
        let to = bytes[2];

        Move::Piece(Unmarshal::coord(from), Unmarshal::coord(to))
    }

    fn decode_promotion(bytes: &[u8]) -> Move {
        let from = bytes[1];
        let to = bytes[2];
        let piece = bytes[3];

        Move::Promotion(
            Unmarshal::coord(from),
            Unmarshal::coord(to),
            Unmarshal::piece(piece),
        )
    }

    fn decode_ksc(bytes: &[u8]) -> Move {
        Move::KingSideCastle(Unmarshal::color(bytes[1]))
    }

    fn decode_qsc(bytes: &[u8]) -> Move {
        Move::QueenSideCastle(Unmarshal::color(bytes[1]))
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
