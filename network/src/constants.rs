/*
* This is a list of INITIAL BITS used to marshal/unmarshal the
*/

pub const BUFFER_BYTE: u8 = 0;

pub const COLOR_WHITE: u8 = 0b0000_0000;
pub const COLOR_BLACK: u8 = 0b0000_0001;

pub const PIECE_PAWN: u8 = 0b0000_0000;
pub const PIECE_KNIGHT: u8 = 0b0001_0000;
pub const PIECE_BISHOP: u8 = 0b0010_0000;
pub const PIECE_ROOK: u8 = 0b0011_0000;
pub const PIECE_QUEEN: u8 = 0b0100_0000;
pub const PIECE_KING: u8 = 0b0101_0000;

// Leading commands
pub const MOVE_PIECE: u8 = 0b0000_0000;
pub const MOVE_PROMOTION: u8 = 0b0000_0001;
pub const MOVE_KING_SIDE_CASTLE: u8 = 0b0000_0010;
pub const MOVE_QUEEN_SIDE_CASTLE: u8 = 0b0000_0011;

pub const GAME_JOIN: u8 = 0b0001_0000;
pub const GAME_LEAVE: u8 = 0b0010_0000;
pub const GAME_RESIGN: u8 = 0b0011_0000;
