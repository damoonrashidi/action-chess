use crate::{
    command::Command,
    constants::{
        BUFFER_BYTE, MOVE_KING_SIDE_CASTLE, MOVE_PIECE, MOVE_PROMOTION, MOVE_QUEEN_SIDE_CASTLE,
    },
};
use state::piece::Move;

use super::{
    constants::{GAME_JOIN, GAME_LEAVE, GAME_RESIGN},
    game_command::GameCmd,
};

impl From<GameCmd> for Command {
    fn from(value: GameCmd) -> Self {
        match value {
            #[allow(clippy::cast_possible_truncation)]
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

struct NetworkMove(Move);

impl From<NetworkMove> for Command {
    fn from(value: NetworkMove) -> Self {
        match value {
            Move::Piece(from, to) => [MOVE_PIECE, from.into(), to.into(), BUFFER_BYTE],
            Move::Promotion(from, to, piece) => {
                [MOVE_PROMOTION, from.into(), to.into(), piece.into()]
            }
            Move::KingSideCastle(color) => [
                MOVE_KING_SIDE_CASTLE,
                color.into(),
                BUFFER_BYTE,
                BUFFER_BYTE,
            ],
            Move::QueenSideCastle(color) => [
                MOVE_QUEEN_SIDE_CASTLE,
                color.into(),
                BUFFER_BYTE,
                BUFFER_BYTE,
            ],
        }
    }
}
