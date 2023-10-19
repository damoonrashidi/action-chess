use super::{
    board::Board,
    coordinate::Coord,
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

    pub fn for_pawn(&self, piece: &Piece, _position: &Coord) -> Vec<Move> {
        let _color = piece.get_color();

        // first check if piece can move two steps
        // then check if piece can move one step
        // then check if piece can capture left
        // then check if piece can capture right
        // finally check promotions

        vec![]
    }

    pub fn for_knight(&self, piece: &Piece, pos: &Coord) -> Vec<Move> {
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

            if let Some(piece) = self.board.get_piece_at(&Coord(*f, *r)) {
                return piece.get_color() != color;
            }

            true
        })
        .map(|(f, r)| Coord(f, r))
        .map(|coord| Move::Piece(*pos, coord))
        .collect()
    }

    pub fn for_rook(&self, piece: &Piece, pos: &Coord) -> Vec<Move> {
        let mut moves = vec![];

        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .for_each(|(y, x)| {
                let mut coord = Coord(pos.0, pos.1);

                while (0..=7).contains(&(coord.0 + y)) && (0..=7).contains(&(coord.1 + x)) {
                    coord = Coord(coord.0 + y, coord.1 + x);
                    if let Some(target) = self.board.get_piece_at(&coord) {
                        if target.get_color() == piece.get_color() {
                            break;
                        } else {
                            moves.push(Move::Piece(*pos, coord));
                            break;
                        }
                    }
                    moves.push(Move::Piece(*pos, coord));
                }
            });

        moves
    }

    pub fn for_bishop(&self, piece: &Piece, pos: &Coord) -> Vec<Move> {
        let mut moves = vec![];

        [(-1, -1), (-1, 1), (1, 1), (1, -1)]
            .iter()
            .for_each(|(y, x)| {
                let mut coord = Coord(pos.0, pos.1);

                while (0..=7).contains(&(coord.0 + y)) && (0..=7).contains(&(coord.1 + x)) {
                    coord = Coord(coord.0 + y, coord.1 + x);
                    if let Some(target) = self.board.get_piece_at(&coord) {
                        if target.get_color() == piece.get_color() {
                            break;
                        } else {
                            moves.push(Move::Piece(*pos, coord));
                            break;
                        }
                    }
                    moves.push(Move::Piece(*pos, coord));
                }
            });

        moves
    }
}
