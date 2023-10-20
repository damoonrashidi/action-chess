use crate::state::piece::Color;

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
        let mut moves = vec![];

        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.board.pieces[y][x] {
                    let coord = Coord(y as i8, x as i8);
                    match piece {
                        Piece::Pawn(_) => moves.extend(&self.for_pawn(&piece, &coord)),
                        Piece::Knight(_) => moves.extend(&self.for_knight(&piece, &coord)),
                        Piece::Bishop(_) => moves.extend(&self.for_bishop(&piece, &coord)),
                        Piece::Rook(_) => moves.extend(&self.for_rook(&piece, &coord)),
                        Piece::Queen(_) => moves.extend(&self.for_queen(&piece, &coord)),
                        Piece::King(_) => moves.extend(&self.for_king(&piece, &coord)),
                    }
                }
            }
        }

        moves
    }

    pub fn for_pawn(&self, piece: &Piece, pos: &Coord) -> Vec<Move> {
        let mut moves = vec![];
        let mut can_move_one = false;
        let (dir, start_rank, promotion_rank) = if piece.get_color() == Color::White {
            (1, 1, 7)
        } else {
            (-1, 6, 0)
        };

        // first check captures
        let left_capture = Coord(pos.0 - 1, pos.1 + dir);
        let right_capture = Coord(pos.0 + 1, pos.1 + dir);
        let forward = Coord(pos.0, pos.1 + dir);
        let double_forward = Coord(pos.0, pos.1 + dir * 2);

        if let (true, Some(target)) = (
            left_capture.is_valid(),
            self.board.get_piece_at(&left_capture),
        ) {
            if target.get_color() != piece.get_color() {
                if left_capture.1 == promotion_rank {
                    moves.extend(MoveGen::all_promotions_at_pos(pos, &left_capture, &piece));
                } else {
                    moves.push(Move::Piece(*pos, left_capture));
                }
            }
        }

        if let (true, Some(target)) = (
            right_capture.is_valid(),
            self.board.get_piece_at(&right_capture),
        ) {
            if target.get_color() != piece.get_color() {
                if right_capture.1 == promotion_rank {
                    moves.extend(MoveGen::all_promotions_at_pos(pos, &right_capture, &piece));
                } else {
                    moves.push(Move::Piece(*pos, right_capture));
                }
            }
        }

        // check forward movement
        if forward.is_valid() && self.board.get_piece_at(&forward).is_none() {
            can_move_one = true;
            if forward.1 == promotion_rank {
                moves.extend(MoveGen::all_promotions_at_pos(pos, &forward, &piece));
            } else {
                moves.push(Move::Piece(*pos, forward));
            }
        }

        // check double move
        if can_move_one && pos.1 == start_rank && self.board.get_piece_at(&double_forward).is_none()
        {
            moves.push(Move::Piece(*pos, double_forward));
        }

        moves
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

                while (0..8).contains(&(coord.0 + y)) && (0..8).contains(&(coord.1 + x)) {
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

                while (0..8).contains(&(coord.0 + y)) && (0..8).contains(&(coord.1 + x)) {
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

    pub fn for_queen(&self, piece: &Piece, pos: &Coord) -> Vec<Move> {
        [self.for_bishop(piece, pos), self.for_rook(piece, pos)]
            .into_iter()
            .flatten()
            .collect()
    }

    pub fn for_king(&self, piece: &Piece, pos: &Coord) -> Vec<Move> {
        (-1..1)
            .flat_map(|i| (-1..1).map(move |j| (i, j)))
            .filter(|(y, x)| (y, x) != (&0, &0) && !(0..8).contains(y) && !(0..8).contains(x))
            .filter(|(y, x)| {
                if let Some(target) = self.board.get_piece_at(&Coord(*y, *x)) {
                    return target.get_color() != piece.get_color();
                }
                true
            })
            .filter(|(y, x)| {
                let _target_square = Coord(*y, *x);
                let _all_enemy_pieces = self.board.get_pieces_by_color(piece.opposing_color());
                false
            })
            .map(|(y, x)| Move::Piece(*pos, Coord(y, x)))
            .collect()
    }

    fn all_promotions_at_pos(from: &Coord, to: &Coord, piece: &Piece) -> Vec<Move> {
        [
            Piece::Queen(piece.get_color()),
            Piece::Rook(piece.get_color()),
            Piece::Bishop(piece.get_color()),
            Piece::Knight(piece.get_color()),
        ]
        .into_iter()
        .map(|p| Move::Promotion(*from, *to, p))
        .collect()
    }
}
