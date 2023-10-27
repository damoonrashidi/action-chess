use crate::state::coordinate::Coord;
use std::{fmt::Display, time::Duration};

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Pawn(Color, Duration),
    Knight(Color, Duration),
    Bishop(Color, Duration),
    Rook(Color, Duration),
    Queen(Color, Duration),
    King(Color, Duration),
}

impl Piece {
    #[inline]
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King(_, _))
    }

    #[inline]
    pub fn is_queen(&self) -> bool {
        matches!(self, Self::Queen(_, _))
    }

    #[inline]
    pub fn is_rook(&self) -> bool {
        matches!(self, Self::Rook(_, _))
    }

    #[inline]
    pub fn is_bishop(&self) -> bool {
        matches!(self, Self::Bishop(_, _))
    }

    #[inline]
    pub fn is_knight(&self) -> bool {
        matches!(self, Self::Knight(_, _))
    }

    #[inline]
    pub fn is_pawn(&self) -> bool {
        matches!(self, Self::Pawn(_, _))
    }

    #[inline]
    pub fn get_color(&self) -> Color {
        match self {
            Piece::Pawn(color, _)
            | Piece::Knight(color, _)
            | Piece::Bishop(color, _)
            | Piece::Rook(color, _)
            | Piece::Queen(color, _)
            | Piece::King(color, _) => *color,
        }
    }

    #[inline]
    pub fn opposing_color(&self) -> Color {
        match self.get_color() {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline]
    pub fn get_cooldown(&self) -> Duration {
        match self {
            Piece::Pawn(_, cd)
            | Piece::Knight(_, cd)
            | Piece::Bishop(_, cd)
            | Piece::Rook(_, cd)
            | Piece::Queen(_, cd)
            | Piece::King(_, cd) => *cd,
        }
    }

    #[inline]
    pub fn set_cooldown(&mut self, cooldown: Duration) {
        *self = match self {
            Piece::Pawn(c, _) => Piece::Pawn(*c, cooldown),
            Piece::Knight(c, _) => Piece::Knight(*c, cooldown),
            Piece::Bishop(c, _) => Piece::Bishop(*c, cooldown),
            Piece::Rook(c, _) => Piece::Rook(*c, cooldown),
            Piece::Queen(c, _) => Piece::Queen(*c, cooldown),
            Piece::King(c, _) => Piece::King(*c, cooldown),
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
            && self.get_color() == other.get_color()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
            Piece::Pawn(color, _) => match color {
                Color::White => "♙",
                Color::Black => "♟︎",
            },
            Piece::Knight(color, _) => match color {
                Color::White => "♘",
                Color::Black => "♞",
            },
            Piece::Bishop(color, _) => match color {
                Color::White => "♗",
                Color::Black => "♝",
            },
            Piece::Rook(color, _) => match color {
                Color::White => "♖",
                Color::Black => "♜",
            },
            Piece::Queen(color, _) => match color {
                Color::White => "♕",
                Color::Black => "♛",
            },
            Piece::King(color, _) => match color {
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
