use super::{
    board::Board,
    coordinate::Coordinate,
    piece::{Move, Piece},
};

pub struct MoveGen<'board> {
    board: &'board Board,
}

impl<'board> MoveGen<'board> {
    pub fn new(board: &'board Board) -> Self {
        Self { board }
    }

    pub fn get_possible_moves(&self) -> Vec<Move> {
        vec![]
    }

    pub fn get_moves_for_pawn(&self, piece: &Piece, position: &Coordinate) -> Vec<Move> {
        let color = piece.get_color();

        // first check if piece can move two steps
        // then check if piece can move one step
        // then check if piece can capture left
        // then check if piece can capture right
        // finally check promotions

        vec![]
    }

    pub fn get_moves_for_knight(&self, piece: &Piece, pos: &Coordinate) -> Vec<Move> {
        let color = piece.get_color();

        let rank = pos.1 as i8;
        let file = pos.0 as i8;

        [
            (file - 1, rank - 2),
            (file + 1, rank - 2),
            (file - 2, rank - 1),
            (file + 2, rank - 1),
            (file - 2, rank + 1),
            (file + 2, rank + 1),
            (file - 1, rank + 2),
            (file + 1, rank + 2),
        ]
        .into_iter()
        .filter(|(f, r)| {
            let coord_is_valid = (0..7).contains(f) && (0..7).contains(r);
            if !coord_is_valid {
                return false;
            }

            if let Some(piece) = self.board.get_piece_at(Coordinate(*f as u8, *r as u8)) {
                return piece.get_color() != color;
            }

            true
        })
        .map(|(f, r)| Coordinate(f as u8, r as u8))
        .map(|coord| Move::Piece(*pos, coord))
        .collect()
    }

    pub fn get_moves_for_rook(&self, piece: &Piece, pos: &Coordinate) -> Vec<Move> {
        let mut moves = vec![];

        moves
    }
}
