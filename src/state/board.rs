use super::{
    coordinate::Coord,
    piece::{Color, Move, Piece},
};
use core::fmt;

#[derive(Debug, Clone, Default)]
#[allow(unused)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],

    white_can_castle_kingside: bool,
    white_can_castle_queenside: bool,
    black_can_castle_kingside: bool,
    black_can_castle_queenside: bool,
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

    pub fn get_piece_at(&self, position: &Coord) -> &Option<Piece> {
        &self.pieces[position.1 as usize][position.0 as usize]
    }

    pub fn set_piece_at(&mut self, piece: Option<Piece>, position: Coord) {
        self.pieces[position.1 as usize][position.0 as usize] = piece
    }

    pub fn get_pieces_by_color(&self, color: Color) -> Vec<(&Piece, Coord)> {
        self.pieces
            .iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, piece)| {
                if let Some(piece) = piece {
                    if (piece.get_color() == color) {
                        return Some((piece, Coord((i % 8) as i8, (i / 8) as i8)));
                    }
                }
                None
            })
            .collect()
    }

    pub fn process_move(&mut self, m: Move) -> &Self {
        if !self.is_valid_move(m) {
            return self;
        }

        match m {
            Move::Piece(from, to) => {
                if let Some(piece) = self.get_piece_at(&from) {
                    self.set_piece_at(Some(*piece), to);
                    self.set_piece_at(None, from);
                }
            }
            Move::KingSideCastle(color) => {
                let src_file = if color == Color::White {
                    self.white_can_castle_kingside = false;
                    0
                } else {
                    self.black_can_castle_kingside = false;
                    7
                };
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

    fn _coordinate_for_file_rank(file: char, rank: char) -> Option<Coord> {
        let file_no = file.to_digit(36)? - 10;
        let rank_no = rank.to_digit(10)? - 1;

        if !(0..7).contains(&file_no) || !(0..7).contains(&rank_no) {
            return None;
        }

        Some(Coord(file_no as i8, rank_no as i8))
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
