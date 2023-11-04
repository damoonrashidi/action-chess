use crate::state::{
    cooldowns::*,
    coordinate::Coord,
    piece::{Color, Move, Piece},
};

const BUFFER_BYTE: u8 = 0b0000_0000;

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

/**
A command is 4 bytes represented by a `[u8; 4]`.
This is all data needed to represent a move, and thus will be all that is sent over the wire during a game.

The first byte indicatates what kind of move it is.

0. Piece
1. Promotion
2. King Side Castle
3. Queen Side Castle

### In the case of (0) Piece.
The second byte is the "from" `Coord` in the format u8 0..64 translated to (file, rank)
The third byte is the "to" `Coord` in the format u8 0..64 translated to (file, rank)
The fourth byte is an EMPTY buffer byte

### In the case of (1) Promotion:

The second byte is the "from" `Coord` sent as a u8 0..64 translated to (file, rank).

The third byte is the "to" `Coord` sent as a u8 0..64 translated to (file, rank).

The forth byte is a u8 representing a Piece where the 4 most significant bits are a piece representation.

```markdown
Pawn = 1
Knight = 2
Bishop = 3
Rook = 4
Queen = 5
King = 6
```
**Note: Pawn and King promotions are illegal moves.**

The four least significant bits are a color representation:
```markdown
0 = White
1 = Black
```
Thus `0b0101_0001` is a black queen.

### In the case of (2) King Side Castle

The second byte is color, signified by the least important bit:
```markdown
0 = White
1 = Black
```
The third byte is an EMPTY buffer byte.

The fourth byte is an EMPTY buffer byte.

### In the case of (2) Queen Side Castle

The second byte is color, signified by the least important bit:
```markdown
0 = White
1 = Black
```
The third byte is an EMPTY buffer byte.

The fourth byte is an EMPTY buffer byte.

*/
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Command(pub [u8; 4]);

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

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        if value & COLOR_BLACK == COLOR_BLACK {
            Color::Black
        } else {
            Color::White
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

impl From<Command> for Move {
    fn from(value: Command) -> Self {
        let bytes = value.0;

        match bytes {
            [MOVE_PIECE, _, _, _] => decode_move(&bytes),
            [MOVE_PROMOTION, _, _, _] => decode_promotion(&bytes),
            [MOVE_KING_SIDE_CASTLE, _, _, _] => decode_ksc(&bytes),
            [MOVE_QUEEN_SIDE_CASTLE, _, _, _] => decode_qsc(&bytes),
            [cmd, _, _, _] => panic!("invalid command {cmd}"),
        }
    }
}

fn decode_move(bytes: &[u8; 4]) -> Move {
    let from = bytes[1];
    let to = bytes[2];

    Move::Piece(from.into(), to.into())
}

fn decode_promotion(bytes: &[u8; 4]) -> Move {
    let from = bytes[1];
    let to = bytes[2];
    let piece = bytes[3];

    Move::Promotion(from.into(), to.into(), piece.into())
}

fn decode_ksc(bytes: &[u8; 4]) -> Move {
    Move::KingSideCastle(bytes[1].into())
}

fn decode_qsc(bytes: &[u8; 4]) -> Move {
    Move::QueenSideCastle(bytes[1].into())
}
