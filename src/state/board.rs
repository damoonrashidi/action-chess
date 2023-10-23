use super::{
    coordinate::Coord,
    piece::{Color, Move, Piece},
};
use core::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
#[allow(unused)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],

    pub white_can_castle_kingside: bool,
    pub white_can_castle_queenside: bool,
    pub black_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,
}

#[allow(unused)]
impl Board {
    pub fn new() -> Self {
        let mut pieces: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        pieces[1] = [Some(Piece::Pawn(Color::White)); 8];
        pieces[6] = [Some(Piece::Pawn(Color::Black)); 8];

        pieces[0] = [
            Some(Piece::Rook(Color::White)),
            Some(Piece::Knight(Color::White)),
            Some(Piece::Bishop(Color::White)),
            Some(Piece::Queen(Color::White)),
            Some(Piece::King(Color::White)),
            Some(Piece::Bishop(Color::White)),
            Some(Piece::Knight(Color::White)),
            Some(Piece::Rook(Color::White)),
        ];

        pieces[7] = [
            Some(Piece::Rook(Color::Black)),
            Some(Piece::Knight(Color::Black)),
            Some(Piece::Bishop(Color::Black)),
            Some(Piece::Queen(Color::Black)),
            Some(Piece::King(Color::Black)),
            Some(Piece::Bishop(Color::Black)),
            Some(Piece::Knight(Color::Black)),
            Some(Piece::Rook(Color::Black)),
        ];

        Self {
            pieces,
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
        }
    }

    pub fn from_fen(fen: &str) -> Option<Self> {
        let parts = fen.split_whitespace().collect::<Vec<&str>>();
        let positions = parts.first()?;
        let mut pieces: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        let mut file = 0;
        let mut rank = 7;
        for c in positions.chars() {
            if c == '/' {
                file = 0;
                rank -= 1;
            }

            if ('0'..'8').contains(&c) {
                if let Ok(empties) = c.to_string().parse::<usize>() {
                    for i in 0..empties {
                        pieces[rank][file] = None;
                        file += 1;
                        continue;
                    }
                }
            }
            let piece = match c {
                'p' => Some(Piece::Pawn(Color::Black)),
                'b' => Some(Piece::Bishop(Color::Black)),
                'n' => Some(Piece::Knight(Color::Black)),
                'r' => Some(Piece::Rook(Color::Black)),
                'q' => Some(Piece::Queen(Color::Black)),
                'k' => Some(Piece::King(Color::Black)),
                'P' => Some(Piece::Pawn(Color::White)),
                'B' => Some(Piece::Bishop(Color::White)),
                'N' => Some(Piece::Knight(Color::White)),
                'R' => Some(Piece::Rook(Color::White)),
                'Q' => Some(Piece::Queen(Color::White)),
                'K' => Some(Piece::King(Color::White)),
                _ => None,
            };
            if file < 8 {
                pieces[rank][file] = piece;
                if piece.is_some() {
                    file += 1;
                }
            }
        }

        let castle_rights = parts.get(2)?;
        let mut black_kingside = false;
        let mut white_kingside = false;
        let mut black_queenside = false;
        let mut white_queenside = false;

        for c in castle_rights.chars() {
            match c {
                'k' => black_kingside = true,
                'K' => white_kingside = true,
                'q' => black_queenside = true,
                'Q' => white_queenside = true,
                _ => {}
            }
        }

        Some(Board {
            pieces,
            white_can_castle_kingside: white_kingside,
            white_can_castle_queenside: white_queenside,
            black_can_castle_kingside: black_kingside,
            black_can_castle_queenside: black_queenside,
        })
    }

    pub fn get_piece_at(&self, position: &Coord) -> &Option<Piece> {
        if !position.is_valid() {
            return &None;
        }
        &self.pieces[position.1 as usize][position.0 as usize]
    }

    pub fn set_piece_at(&mut self, piece: Option<Piece>, position: Coord) {
        self.pieces[position.1 as usize][position.0 as usize] = piece
    }

    pub fn remove_by_piece(&mut self, piece: &Piece) {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(target) = self.pieces[y][x] {
                    if std::mem::discriminant(&target) == std::mem::discriminant(piece)
                        && piece.get_color() == target.get_color()
                    {
                        self.pieces[y][x] = None;
                    }
                }
            }
        }
    }

    pub fn filter_by_piece(&self, piece: Piece) -> Board {
        let mut pieces = [[None; 8]; 8];
        for y in 0..8 {
            for x in 0..8 {
                pieces[y][x] = match (piece, self.pieces[y][x]) {
                    (_, None) => None,
                    (comp, Some(target)) => {
                        if comp == target {
                            Some(target)
                        } else {
                            None
                        }
                    }
                };
            }
        }

        Board {
            pieces,
            ..self.clone()
        }
    }

    pub fn filter_by_color(&self, color: Color) -> Board {
        let mut pieces = [[None; 8]; 8];

        for y in 0..8 {
            for x in 0..8 {
                pieces[y][x] = match (color, self.pieces[y][x]) {
                    (_, None) => None,
                    (comp, Some(target)) => {
                        if comp == target.get_color() {
                            Some(target)
                        } else {
                            None
                        }
                    }
                };
            }
        }

        Board {
            pieces,
            ..self.clone()
        }
    }

    pub fn process_move(&mut self, m: Move) -> &Self {
        if !self.is_valid_move(m) {
            return self;
        }

        match m {
            Move::Piece(from, to) => {
                let piece = self.get_piece_at(&from);
                self.set_piece_at(*piece, to);
                self.set_piece_at(None, from);
            }
            Move::KingSideCastle(color) => {
                let src_file = if color == Color::White { 0 } else { 7 };
                if color == Color::White {
                    self.white_can_castle_kingside = false;
                    self.white_can_castle_queenside = false;
                } else {
                    self.black_can_castle_kingside = false;
                    self.black_can_castle_queenside = false;
                }
                self.pieces[src_file][4] = None;
                self.pieces[src_file][7] = None;
                self.pieces[src_file][6] = Some(Piece::King(color));
                self.pieces[src_file][5] = Some(Piece::Rook(color));
            }
            Move::QueenSideCastle(color) => {
                let src_file = if color == Color::White { 0 } else { 7 };
                self.pieces[src_file][4] = None;
                self.pieces[src_file][0] = None;
                self.pieces[src_file][2] = Some(Piece::King(color));
                self.pieces[src_file][2] = Some(Piece::Rook(color));
            }
            Move::Promotion(src, dest, piece) => {
                self.pieces[src.1 as usize][src.0 as usize] = None;
                self.pieces[dest.1 as usize][dest.0 as usize] = Some(piece);
            }
        }

        self
    }

    pub fn get_coord_for_piece(&self, piece: &Piece) -> Option<Coord> {
        for y in 0..8 {
            for x in 0..8 {
                if self.pieces[y][x] == Some(*piece) {
                    return Some(Coord(x as i8, y as i8));
                }
            }
        }
        None
    }

    fn is_valid_move(&self, mv: Move) -> bool {
        match mv {
            Move::KingSideCastle(color) => match color {
                Color::White => self.white_can_castle_kingside,
                Color::Black => self.black_can_castle_kingside,
            },
            Move::QueenSideCastle(color) => match color {
                Color::White => self.white_can_castle_queenside,
                Color::Black => self.black_can_castle_queenside,
            },
            Move::Piece(_src, _dest) => true,
            Move::Promotion(_src, _dest, _piece) => true,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = String::from("");

        for y in (0..8).rev() {
            board = format!("{board}{} ", y + 1);
            for x in 0..8 {
                match self.pieces[y][x] {
                    Some(piece) => board = format!("{board}{piece}"),
                    None => board = format!("{board} "),
                }
            }
            board = format!("{board}\n");
        }
        write!(f, "{board}\n  abcdefgh")
    }
}
