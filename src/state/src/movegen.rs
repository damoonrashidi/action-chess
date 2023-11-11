use std::{
    sync::{mpsc::channel, Arc},
    time::Duration,
};

use threadpool::ThreadPool;

use super::{
    board::Board,
    cooldowns::{COOLDOWN_BISHOP, COOLDOWN_KING, COOLDOWN_KNIGHT, COOLDOWN_QUEEN, COOLDOWN_ROOK},
    coordinate::Coord,
    piece::{Move, Piece},
};
use crate::piece::Color;
use colored::Colorize;

pub struct MoveGen<'board> {
    board: &'board Board,
}

impl<'board> MoveGen<'board> {
    pub fn new(board: &'board Board) -> Self {
        Self { board }
    }

    pub fn get_possible_moves(&self) -> Vec<Move> {
        let num_threads = self.board.get_piece_count();
        let pool = ThreadPool::new(num_threads);
        let board = Arc::new(self.board.clone());
        let (s, r) = channel::<Vec<Move>>();

        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.board.pieces[y][x] {
                    let coord = Coord(x as i8, y as i8);
                    let s = s.clone();
                    let board = board.clone();
                    pool.execute(move || {
                        let gen = MoveGen::new(&board);
                        let moves = match piece {
                            Piece::Pawn(_, _) => gen.for_pawn(&piece, coord),
                            Piece::Knight(_, _) => gen.for_knight(&piece, coord),
                            Piece::Bishop(_, _) => gen.for_bishop(&piece, coord),
                            Piece::Rook(_, _) => gen.for_rook(&piece, coord),
                            Piece::Queen(_, _) => gen.for_queen(&piece, coord),
                            Piece::King(_, _) => gen.for_king(&piece, coord),
                        };
                        s.send(moves).unwrap();
                    });
                }
            }
        }

        r.iter().take(num_threads).flatten().collect()
    }

    pub fn get_possible_moves_for_color(&self, color: Color) -> Vec<Move> {
        let num_pieces_with_color: usize = self.board.get_piece_count();
        let pool = ThreadPool::new(num_pieces_with_color);
        let (s, r) = channel::<Vec<Move>>();
        let board = Arc::new(self.board.clone());

        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.board.pieces[y][x] {
                    let s = s.clone();
                    let board = board.clone();
                    pool.execute(move || {
                        let coord = Coord(x as i8, y as i8);
                        let gen = MoveGen::new(&board);
                        let moves = match (piece, color == piece.get_color()) {
                            (Piece::Pawn(_, _), true) => gen.for_pawn(&piece, coord),
                            (Piece::Knight(_, _), true) => gen.for_knight(&piece, coord),
                            (Piece::Bishop(_, _), true) => gen.for_bishop(&piece, coord),
                            (Piece::Rook(_, _), true) => gen.for_rook(&piece, coord),
                            (Piece::Queen(_, _), true) => gen.for_queen(&piece, coord),
                            (Piece::King(_, _), true) => gen.for_king(&piece, coord),
                            _ => vec![],
                        };
                        s.send(moves).unwrap();
                    });
                }
            }
        }

        r.iter().take(num_pieces_with_color).flatten().collect()
    }

    pub fn for_pawn(&self, piece: &Piece, pos: Coord) -> Vec<Move> {
        if !piece.get_cooldown().is_zero() {
            return vec![];
        }
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

        if let (Some(target), true) = (
            self.board.get_piece_at(&left_capture),
            left_capture.is_valid(),
        ) {
            if target.get_color() != piece.get_color() {
                if left_capture.1 == promotion_rank {
                    moves.extend(MoveGen::all_promotions_at_pos(pos, left_capture, piece));
                } else {
                    moves.push(Move::Piece(pos, left_capture));
                }
            }
        }

        if let (Some(target), true) = (
            self.board.get_piece_at(&right_capture),
            right_capture.is_valid(),
        ) {
            if target.get_color() != piece.get_color() {
                if right_capture.1 == promotion_rank {
                    moves.extend(MoveGen::all_promotions_at_pos(pos, right_capture, piece));
                } else {
                    moves.push(Move::Piece(pos, right_capture));
                }
            }
        }

        // check forward movement
        if forward.is_valid() && self.board.get_piece_at(&forward).is_none() {
            can_move_one = true;
            if forward.1 == promotion_rank {
                moves.extend(MoveGen::all_promotions_at_pos(pos, forward, piece));
            } else {
                moves.push(Move::Piece(pos, forward));
            }
        }

        // check double move
        if can_move_one && pos.1 == start_rank && self.board.get_piece_at(&double_forward).is_none()
        {
            moves.push(Move::Piece(pos, double_forward));
        }

        moves
            .into_iter()
            .filter(|m| !self.move_would_result_in_check(m, piece))
            .collect()
    }

    pub fn for_knight(&self, piece: &Piece, pos: Coord) -> Vec<Move> {
        if !piece.get_cooldown().is_zero() {
            return vec![];
        }
        let color = piece.get_color();

        let rank = pos.1;
        let file = pos.0;

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
        .map(|(f, r)| Coord(f, r))
        .filter(|coord| {
            if !coord.is_valid() {
                return false;
            }

            if let Some(piece) = self.board.get_piece_at(coord) {
                return piece.get_color() != color;
            }

            true
        })
        .map(|coord| Move::Piece(pos, coord))
        .filter(|m| !self.move_would_result_in_check(m, piece))
        .collect()
    }

    pub fn for_rook(&self, piece: &Piece, pos: Coord) -> Vec<Move> {
        if !piece.get_cooldown().is_zero() {
            return vec![];
        }

        let mut moves = vec![];

        for (y, x) in &[(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let mut coord = Coord(pos.0, pos.1);

            while (0..8).contains(&(coord.0 + y)) && (0..8).contains(&(coord.1 + x)) {
                coord = Coord(coord.0 + y, coord.1 + x);
                if coord == pos {
                    continue;
                }
                if let Some(target) = self.board.get_piece_at(&coord) {
                    if target.get_color() != piece.get_color() {
                        moves.push(Move::Piece(pos, coord));
                    }
                    break;
                }
                moves.push(Move::Piece(pos, coord));
            }
        }

        moves
            .into_iter()
            .filter(|m| !self.move_would_result_in_check(m, piece))
            .collect()
    }

    pub fn for_bishop(&self, piece: &Piece, pos: Coord) -> Vec<Move> {
        if !piece.get_cooldown().is_zero() {
            return vec![];
        }
        let mut moves = vec![];

        for (y, x) in &[(-1, -1), (-1, 1), (1, 1), (1, -1)] {
            let mut coord = Coord(pos.0, pos.1);

            while (0..8).contains(&(coord.0 + y)) && (0..8).contains(&(coord.1 + x)) {
                coord = Coord(coord.0 + y, coord.1 + x);
                if coord == pos {
                    continue;
                }
                if let Some(target) = self.board.get_piece_at(&coord) {
                    if target.get_color() != piece.get_color() {
                        moves.push(Move::Piece(pos, coord));
                    }
                    break;
                }
                moves.push(Move::Piece(pos, coord));
            }
        }

        moves
            .into_iter()
            .filter(|m| !self.move_would_result_in_check(m, piece))
            .collect()
    }

    pub fn for_queen(&self, piece: &Piece, pos: Coord) -> Vec<Move> {
        [self.for_bishop(piece, pos), self.for_rook(piece, pos)]
            .into_iter()
            .flatten()
            .collect()
    }

    pub fn for_king(&self, piece: &Piece, pos: Coord) -> Vec<Move> {
        if !piece.get_cooldown().is_zero() {
            return vec![];
        }
        let mut natural_moves: Vec<Move> = (-1..=1)
            .flat_map(|i| (-1..=1).map(move |j| Coord(pos.0 + i, pos.1 + j)))
            .filter(|coord| coord.is_valid() && coord != &pos)
            .filter(|coord| {
                if let Some(opposing_king_pos) = self
                    .board
                    .get_coord_for_piece(&Piece::King(piece.opposing_color(), COOLDOWN_KING))
                {
                    let file_distance = (opposing_king_pos.0 - coord.0).abs();
                    let rank_distance = (opposing_king_pos.1 - coord.1).abs();
                    if file_distance <= 1 && rank_distance <= 1 {
                        return false;
                    }
                }

                if let Some(target) = self.board.get_piece_at(coord) {
                    return target.get_color() != piece.get_color();
                }
                true
            })
            .filter(|coord| {
                let mut enemy_board = self.board.filter_by_color(piece.opposing_color());
                enemy_board.set_piece_at(Some(*piece), *coord);
                enemy_board.remove_by_piece(&Piece::King(piece.opposing_color(), COOLDOWN_KING));
                let movegen = MoveGen::new(&enemy_board);
                let enemy_moves = movegen.get_possible_moves_for_color(piece.opposing_color());
                let enemy_can_capture_at_coord = enemy_moves
                    .iter()
                    .filter_map(|m| match m {
                        Move::Promotion(_, target, _) | Move::Piece(_, target) => Some(target),
                        _ => None,
                    })
                    .collect::<Vec<&Coord>>()
                    .contains(&coord);
                !enemy_can_capture_at_coord
            })
            .map(|target| Move::Piece(pos, target))
            .collect();

        let (can_castle_king_side, can_castle_queen_side, castle_rank) =
            if piece.get_color() == Color::White {
                (
                    self.board.white_can_castle_kingside,
                    self.board.white_can_castle_queenside,
                    0,
                )
            } else {
                (
                    self.board.black_can_castle_kingside,
                    self.board.black_can_castle_queenside,
                    7,
                )
            };

        if can_castle_king_side {
            let king_is_starting_position = pos == Coord(4, castle_rank);
            let lane_is_open = [1, 2]
                .iter()
                .map(|file_diff| Coord(4 + file_diff, castle_rank))
                .all(|coord| self.board.get_piece_at(&coord).is_none());
            if let (Some(target), true, true) = (
                self.board.get_piece_at(&Coord(7, castle_rank)),
                lane_is_open,
                king_is_starting_position,
            ) {
                if target == &Piece::Rook(piece.get_color(), COOLDOWN_ROOK) {
                    natural_moves.push(Move::KingSideCastle(piece.get_color()));
                }
            }
        }

        if can_castle_queen_side {
            let king_is_starting_position = pos == Coord(4, castle_rank);
            let lane_is_open = [-1, -2, -3]
                .iter()
                .map(|file_diff| Coord(4 + file_diff, castle_rank))
                .all(|coord| self.board.get_piece_at(&coord).is_none());
            if let (Some(target), true, true) = (
                self.board.get_piece_at(&Coord(0, castle_rank)),
                lane_is_open,
                king_is_starting_position,
            ) {
                if target == &Piece::Rook(piece.get_color(), COOLDOWN_ROOK) {
                    natural_moves.push(Move::QueenSideCastle(piece.get_color()));
                }
            }
        }

        natural_moves
    }

    fn move_would_result_in_check(&self, m: &Move, piece: &Piece) -> bool {
        let mut board = self.board.filter_by_color(piece.opposing_color());
        let king_pos = self
            .board
            .get_coord_for_piece(&Piece::King(piece.get_color(), COOLDOWN_KING));
        if king_pos.is_none() {
            return false;
        }
        match m {
            Move::Promotion(_, to, _) | Move::Piece(_, to) => board.set_piece_at(Some(*piece), *to),
            _ => {}
        }
        let gen = MoveGen::new(&board);
        let moves = gen.get_possible_moves_for_color(piece.opposing_color());

        moves
            .into_iter()
            .filter_map(|m| match m {
                Move::Promotion(_, to, _) | Move::Piece(_, to) => Some(to),
                _ => None,
            })
            .collect::<Vec<Coord>>()
            .contains(&king_pos.unwrap())
    }

    fn all_promotions_at_pos(from: Coord, to: Coord, piece: &Piece) -> Vec<Move> {
        [
            Piece::Queen(piece.get_color(), COOLDOWN_QUEEN),
            Piece::Rook(piece.get_color(), COOLDOWN_ROOK),
            Piece::Bishop(piece.get_color(), COOLDOWN_BISHOP),
            Piece::Knight(piece.get_color(), COOLDOWN_KNIGHT),
        ]
        .into_iter()
        .map(|p| Move::Promotion(from, to, p))
        .collect()
    }

    #[allow(unused)]
    pub fn render_movelist(board: &Board, moves: &[Move]) {
        let mut render = String::from("    A | B | C | D | E | F | G | H \n");
        render = format!("{render}  |---+---+---+---+---+---+---+---|\n");
        let targets: Vec<Coord> = moves
            .iter()
            .filter_map(|m| match m {
                Move::Promotion(_, target, _) | Move::Piece(_, target) => Some(*target),
                _ => None,
            })
            .collect();

        for y in (0..8).rev() {
            render = format!("\n{render}{} ", y + 1);
            for x in 0..8 {
                render = match (
                    board.pieces[y][x],
                    targets.contains(&Coord(x as i8, y as i8)),
                ) {
                    (None, true) => format!("{render}| o "),
                    (None, false) => format!("{render}|   "),
                    (Some(p), true) => {
                        format!(
                            "{render}|{}{}{}",
                            " ".on_yellow(),
                            p.to_string().on_yellow(),
                            " ".to_string().on_yellow()
                        )
                    }
                    (Some(p), false) => {
                        if p.get_cooldown().gt(&Duration::ZERO) {
                            format!(
                                "{render}|{}{}{}",
                                " ".on_red(),
                                p.to_string().on_red(),
                                " ".on_red()
                            )
                        } else {
                            format!("{render}| {p} ")
                        }
                    }
                }
            }
            render = format!("{render}| {}\n  |---+---+---+---+---+---+---+---|\n", y + 1);
        }
        println!("{render}    A | B | C | D | E | F | G | H \n");
    }
}
