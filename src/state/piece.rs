use crate::state::coordinate::Coord;
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn get_color(&self) -> Color {
        match self {
            Piece::Pawn(color)
            | Piece::Knight(color)
            | Piece::Bishop(color)
            | Piece::Rook(color)
            | Piece::Queen(color)
            | Piece::King(color) => *color,
        }
    }

    pub fn is_king(&self) -> bool {
        matches!(self, Self::King(_))
    }

    pub fn is_queen(&self) -> bool {
        matches!(self, Self::Queen(_))
    }

    pub fn is_rook(&self) -> bool {
        matches!(self, Self::Rook(_))
    }

    pub fn is_bishop(&self) -> bool {
        matches!(self, Self::Bishop(_))
    }

    pub fn is_knight(&self) -> bool {
        matches!(self, Self::Knight(_))
    }

    pub fn is_pawn(&self) -> bool {
        matches!(self, Self::Pawn(_))
    }

    pub fn opposing_color(&self) -> Color {
        match self.get_color() {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    KingSideCastle(Color),
    QueenSideCastle(Color),
    Piece(Coord, Coord),
    Promotion(Coord, Coord, Piece),
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mv = match self {
            Move::KingSideCastle(_) => "o-o".into(),
            Move::QueenSideCastle(_) => "o-o-o".into(),
            Move::Piece(from, to) => format!("{from} -> {to}"),
            Move::Promotion(_, to, piece) => format!("{to}={piece}"),
        };

        write!(f, "{mv}")
    }
}

impl From<&Piece> for String {
    fn from(value: &Piece) -> Self {
        let char = match value {
            Piece::Pawn(color) => match color {
                Color::White => "♙",
                Color::Black => "♟︎",
            },
            Piece::Knight(color) => match color {
                Color::White => "♘",
                Color::Black => "♞",
            },
            Piece::Bishop(color) => match color {
                Color::White => "♗",
                Color::Black => "♝",
            },
            Piece::Rook(color) => match color {
                Color::White => "♖",
                Color::Black => "♜",
            },
            Piece::Queen(color) => match color {
                Color::White => "♕",
                Color::Black => "♛",
            },
            Piece::King(color) => match color {
                Color::White => "♔",
                Color::Black => "♚",
            },
        };

        char.into()
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: String = String::from(self);
        write!(f, "{value}")
    }
}
