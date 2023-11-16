/*
* This is a list of INITIAL BITS used to marshal/unmarshal commands, pieces and game_commands
*/

pub(crate) const BUFFER_BYTE: u8 = 0;

pub(crate) const COLOR_WHITE: u8 = 0b0000_0000;
pub(crate) const COLOR_BLACK: u8 = 0b0000_0001;

pub(crate) const PIECE_PAWN: u8 = 0b0000_0000;
pub(crate) const PIECE_KNIGHT: u8 = 0b0001_0000;
pub(crate) const PIECE_BISHOP: u8 = 0b0010_0000;
pub(crate) const PIECE_ROOK: u8 = 0b0011_0000;
pub(crate) const PIECE_QUEEN: u8 = 0b0100_0000;
pub(crate) const PIECE_KING: u8 = 0b0101_0000;

// Leading commands
pub(crate) const MOVE_PIECE: u8 = 0b0000_0000;
pub(crate) const MOVE_PROMOTION: u8 = 0b0000_0001;
pub(crate) const MOVE_KING_SIDE_CASTLE: u8 = 0b0000_0010;
pub(crate) const MOVE_QUEEN_SIDE_CASTLE: u8 = 0b0000_0011;

pub(crate) const GAME_JOIN: u8 = 0b0001_0000;
pub(crate) const GAME_LEAVE: u8 = 0b0010_0000;
pub(crate) const GAME_RESIGN: u8 = 0b0011_0000;
