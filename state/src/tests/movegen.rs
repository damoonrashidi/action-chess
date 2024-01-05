use std::time::Duration;

use crate::board::Board;

impl Board {
    #[must_use]
    pub fn standard_without_cooldowns() -> Self {
        let mut board = Board::standard();
        for y in 0..8 {
            for x in 0..8 {
                if let Some(mut piece) = board.pieces[y][x] {
                    piece.set_cooldown(Duration::ZERO);
                    board.pieces[y][x] = Some(piece);
                }
            }
        }
        board
    }
}

#[cfg(test)]
mod moves {
    use std::time::Duration;

    use crate::{
        board::Board,
        movegen::MoveGen,
        piece::{
            Color::{Black, White},
            Piece,
        },
        square::*,
    };

    #[test]
    fn all_possible_moves() {
        let board = Board::standard_without_cooldowns();
        let moves = MoveGen::new(&board).get_possible_moves();
        assert_eq!(moves.len(), 40);
    }

    #[test]
    fn all_black_starting_moves() {
        let board = Board::standard_without_cooldowns().filter_by_color(Black);
        let moves = MoveGen::new(&board).get_possible_moves();
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn all_black_starting_moves_with_capture() {
        let mut board = Board::standard_without_cooldowns().filter_by_color(Black);
        board.set_piece_at(Some(Piece::Pawn(White, Duration::ZERO)), D6);
        let moves = MoveGen::new(&board).get_possible_moves();

        assert_eq!(moves.len(), 22);
    }

    #[test]
    fn cooldown_on_piece() {
        let mut board = Board::empty();
        board.set_piece_at(Some(Piece::Rook(Black, Duration::ZERO)), F3);
        let moves =
            MoveGen::new(&board).for_rook(&Piece::Rook(White, Duration::from_millis(50)), A3);
        assert!(moves.is_empty());
    }
}

#[cfg(test)]
mod bishop_moves {
    use std::time::Duration;

    use crate::{
        board::Board,
        coordinate::Coord,
        movegen::MoveGen,
        piece::{Color::White, Move, Piece},
        square::*,
    };

    #[test]
    fn bishop_on_empty() {
        let board = Board::empty();
        let moves = MoveGen::new(&board).for_bishop(&Piece::Bishop(White, Duration::ZERO), E5);
        let expected_moves = vec![D4, C3, B2, A1, F6, G7, H8, F4, G3, H2, D6, C7, B8];

        assert!(move_lists_has_all_targets(E5, &expected_moves, &moves));
    }

    #[test]
    fn bishop_on_standard() {
        let board = Board::standard();
        let moves = MoveGen::new(&board).for_bishop(&Piece::Bishop(White, Duration::ZERO), E5);
        let expected_moves = vec![D4, C3, F6, G7, F4, G3, D6, C7];

        assert!(move_lists_has_all_targets(E5, &expected_moves, &moves));
    }

    fn move_lists_has_all_targets(
        start_pos: Coord,
        expected_targets: &[Coord],
        moves: &[Move],
    ) -> bool {
        expected_targets
            .iter()
            .map(|target| Move::Piece(start_pos, *target))
            .all(|m| moves.contains(&m))
    }
}

#[cfg(test)]
mod knight_moves {
    use std::time::Duration;

    use crate::{
        board::Board,
        coordinate::Coord,
        movegen::MoveGen,
        piece::{Color::White, Move, Piece},
        square::*,
    };

    #[test]
    fn knight_in_starting_position() {
        let board = Board::standard();
        let moves = MoveGen::new(&board).for_knight(&Piece::Knight(White, Duration::ZERO), G1);
        let expected_moves = vec![H3, F3];

        assert!(move_lists_has_all_targets(G1, &expected_moves, &moves));
    }

    #[test]
    fn knight_on_standard_board() {
        let board = Board::standard();
        let moves = MoveGen::new(&board).for_knight(&Piece::Knight(White, Duration::ZERO), E4);

        assert_eq!(moves.len(), 6);
    }

    fn move_lists_has_all_targets(
        start_pos: Coord,
        expected_targets: &[Coord],
        moves: &[Move],
    ) -> bool {
        expected_targets
            .iter()
            .map(|target| Move::Piece(start_pos, *target))
            .all(|m| moves.contains(&m))
    }
}

#[cfg(test)]
mod king_moves {
    use crate::{
        board::Board,
        coordinate::Coord,
        movegen::MoveGen,
        piece::{
            Color::{Black, White},
            Move, Piece,
        },
        square::*,
    };
    use std::time::Duration;

    #[test]
    fn king_cant_move_into_pawn_check() {
        let mut board = Board::standard();
        board.set_piece_at(Some(Piece::King(White, Duration::ZERO)), E5);
        for y in 0..8 {
            for x in 0..8 {
                if let Some(mut piece) = board.pieces[x][y] {
                    piece.set_cooldown(Duration::ZERO);
                    board.pieces[x][y] = Some(piece);
                }
            }
        }
        let moves = MoveGen::new(&board).for_king(&Piece::King(White, Duration::ZERO), E5);
        let expected_moves = vec![D5, F5, D4, E4, F4];

        assert!(move_lists_has_all_targets(E5, &expected_moves, &moves));
        assert_eq!(expected_moves.len(), moves.len());
    }

    #[test]
    fn king_on_empty_board() {
        let board = Board::empty();
        let moves = MoveGen::new(&board).for_king(&Piece::King(White, Duration::ZERO), E5);

        assert_eq!(moves.len(), 8);

        assert!(move_lists_has_all_targets(
            E5,
            &[D4, D5, D6, E4, E6, F4, F5, F6],
            &moves
        ));
    }

    #[test]
    fn king_cant_move_into_check() {
        let mut board = Board::empty();
        board.set_piece_at(Some(Piece::Rook(Black, Duration::ZERO)), A1);
        let moves = MoveGen::new(&board).for_king(&Piece::King(White, Duration::ZERO), B2);

        assert!(move_lists_has_all_targets(B2, &[A1, B3, C2, C3], &moves));
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn king_cant_move_into_opposing_king() {
        let mut board = Board::empty();
        board.set_piece_at(Some(Piece::King(Black, Duration::ZERO)), D5);
        let moves = MoveGen::new(&board).for_king(&Piece::King(White, Duration::ZERO), D3);
        let expected_moves = vec![C3, E3, C2, D2, E2];
        assert_eq!(moves.len(), expected_moves.len());
        assert!(move_lists_has_all_targets(D3, &expected_moves, &moves));
    }

    fn move_lists_has_all_targets(
        start_pos: Coord,
        expected_targets: &[Coord],
        moves: &[Move],
    ) -> bool {
        expected_targets
            .iter()
            .map(|target| Move::Piece(start_pos, *target))
            .all(|m| moves.contains(&m))
    }

    #[test]
    fn check_in_standard() {
        let mut board = Board::standard();
        board.set_piece_at(Some(Piece::Pawn(White, Duration::ZERO)), D7);
        let (_, attacks_on_black) = board.king_check_count();

        assert_eq!(attacks_on_black, 1);
    }
}

#[cfg(test)]
mod pawn_moves {
    use std::time::Duration;

    use crate::{
        board::Board,
        coordinate::Coord,
        movegen::MoveGen,
        piece::{
            Color::{Black, White},
            Move, Piece,
        },
        square::*,
    };

    #[test]
    fn pawn_capture() {
        let mut board = Board::empty();
        board.set_piece_at(Some(Piece::Pawn(Black, Duration::ZERO)), D3);
        board.set_piece_at(Some(Piece::Bishop(White, Duration::ZERO)), C3);
        board.set_piece_at(Some(Piece::Rook(White, Duration::ZERO)), E3);
        let moves = MoveGen::new(&board).for_pawn(&Piece::Pawn(Black, Duration::ZERO), D4);
        let expected_moves = vec![C3, E3];

        assert!(move_lists_has_all_targets(D4, &expected_moves, &moves));
    }

    #[test]
    fn white_pawn_at_start() {
        let board = Board::standard();
        let moves = MoveGen::new(&board).for_pawn(&Piece::Pawn(White, Duration::ZERO), A2);
        let expected_moves = vec![A3, A4];

        assert!(move_lists_has_all_targets(A2, &expected_moves, &moves));
    }

    #[test]
    fn black_pawn_at_start_with_capture() {
        let mut board = Board::empty();
        board.set_piece_at(Some(Piece::Bishop(White, Duration::ZERO)), A6);
        let moves = MoveGen::new(&board).for_pawn(&Piece::Pawn(Black, Duration::ZERO), B7);
        let expected_moves = vec![A6, B6, B5];

        assert!(move_lists_has_all_targets(B7, &expected_moves, &moves));
    }

    #[test]
    fn black_pawn_at_start() {
        let mut board = Board::empty();
        board.set_piece_at(Some(Piece::Bishop(White, Duration::ZERO)), D5);
        let moves = MoveGen::new(&board).for_pawn(&Piece::Pawn(Black, Duration::ZERO), D7);
        let expected_moves = vec![D6];

        assert!(move_lists_has_all_targets(D7, &expected_moves, &moves));
    }

    #[test]
    fn promote_white_pawn() {
        let board = Board::empty();
        let moves = MoveGen::new(&board).for_pawn(&Piece::Pawn(Black, Duration::ZERO), B2);
        assert_eq!(moves.len(), 4);
    }

    fn move_lists_has_all_targets(
        start_pos: Coord,
        expected_targets: &[Coord],
        moves: &[Move],
    ) -> bool {
        expected_targets
            .iter()
            .map(|target| Move::Piece(start_pos, *target))
            .all(|m| moves.contains(&m))
    }
}

#[cfg(test)]
mod castling {
    use std::time::Duration;

    use crate::{
        board::Board,
        movegen::MoveGen,
        piece::{Color::White, Move, Piece},
        square::*,
    };

    #[test]
    fn king_castle_kingside() {
        let mut board = Board::empty();
        board.set_piece_at(Some(Piece::Rook(White, Duration::ZERO)), H1);
        board.white_can_castle_kingside = true;
        let moves = MoveGen::new(&board).for_king(&Piece::King(White, Duration::ZERO), E1);
        assert!(moves.contains(&Move::KingSideCastle(White)));
    }

    #[test]
    fn blocked_queenside_castle() {
        let mut board = Board::empty();
        board.set_piece_at(Some(Piece::Queen(White, Duration::ZERO)), D1);
        board.set_piece_at(Some(Piece::Rook(White, Duration::ZERO)), A1);
        board.set_piece_at(Some(Piece::Rook(White, Duration::ZERO)), H1);
        board.white_can_castle_kingside = true;
        board.white_can_castle_queenside = true;
        let moves = MoveGen::new(&board).for_king(&Piece::King(White, Duration::ZERO), E1);
        assert!(moves.contains(&Move::KingSideCastle(White)));
        assert!(!moves.contains(&Move::QueenSideCastle(White)));
    }

    #[test]
    fn moved_king_castle_kingside() {
        let mut board = Board::empty();
        board.set_piece_at(Some(Piece::Rook(White, Duration::ZERO)), H1);
        board.white_can_castle_kingside = true;
        let moves = MoveGen::new(&board).for_king(&Piece::King(White, Duration::ZERO), E2);
        assert!(!moves.contains(&Move::KingSideCastle(White)));
    }

    #[test]
    fn king_castle_queenside() {
        let mut board = Board::empty();
        board.set_piece_at(Some(Piece::Rook(White, Duration::ZERO)), A1);
        board.white_can_castle_queenside = true;
        let moves = MoveGen::new(&board).for_king(&Piece::King(White, Duration::ZERO), E1);
        assert!(moves.contains(&Move::QueenSideCastle(White)));
    }
}

#[cfg(test)]
mod rook_moves {
    use std::time::Duration;

    use crate::{
        board::Board,
        coordinate::Coord,
        movegen::MoveGen,
        piece::{
            Color::{Black, White},
            Move, Piece,
        },
        square::*,
    };

    #[test]
    fn rook_on_empty_board() {
        let board = Board::empty();
        let moves = MoveGen::new(&board).for_rook(&Piece::Rook(White, Duration::ZERO), E4);

        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn rook_on_standard_board() {
        let board = Board::standard();
        let moves = MoveGen::new(&board).for_rook(&Piece::Rook(White, Duration::ZERO), A5);
        let expected_moves = vec![A6, A7, B5, C5, D5, E5, F5, G5, H5, A4, A3];

        assert!(move_lists_has_all_targets(A5, &expected_moves, &moves));
    }

    #[test]
    fn starting_rook() {
        let board = Board::standard();
        let moves = MoveGen::new(&board).for_rook(&Piece::Rook(White, Duration::ZERO), A1);

        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn infiltrated_starting_rook() {
        let board = Board::standard();
        let moves = MoveGen::new(&board).for_rook(&Piece::Rook(Black, Duration::ZERO), A1);

        assert_eq!(vec![Move::Piece(A1, A2), Move::Piece(A1, B1)], moves);
    }

    fn move_lists_has_all_targets(
        start_pos: Coord,
        expected_targets: &[Coord],
        moves: &[Move],
    ) -> bool {
        expected_targets
            .iter()
            .map(|target| Move::Piece(start_pos, *target))
            .all(|m| moves.contains(&m))
    }
}
