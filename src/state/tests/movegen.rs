#[cfg(test)]
mod tests {
    use crate::state::{
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
    fn all_possible_moves() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.get_possible_moves();
        assert_eq!(moves.len(), 40);
    }

    #[test]
    fn king_on_empty_board() {
        let board = Board::default();
        let gen = MoveGen::new(&board);
        let moves = gen.for_king(&Piece::King(White), &A1);

        assert!(move_lists_has_all_targets(A1, &vec![A2, B1, B2], &moves));
    }

    #[test]
    fn king_cant_move_into_check() {
        let mut board = Board::default();
        board.set_piece_at(Some(Piece::Rook(Black)), A1);
        let gen = MoveGen::new(&board);
        let moves = gen.for_king(&Piece::King(White), &B2);

        assert!(move_lists_has_all_targets(
            B2,
            &vec![A1, B3, C2, C3],
            &moves
        ));
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn king_cant_move_into_pawn_check() {
        let board = Board::new();
        let gen = MoveGen::new(&board);
        let moves = gen.for_king(&Piece::King(White), &E5);
        let expected_moves = [D5, F5, D4, E4, F4];
        println!("moves");
        moves.iter().for_each(|m| println!("{m}"));

        assert!(move_lists_has_all_targets(
            E5,
            &expected_moves.into(),
            &moves
        ));
        assert_eq!(expected_moves.len(), moves.len());
    }

    #[test]
    fn king_castle_kingside() {
        let mut board = Board::default();
        board.set_piece_at(Some(Piece::Rook(White)), H1);
        board.white_can_castle_kingside = true;
        let gen = MoveGen::new(&board);
        let moves = gen.for_king(&Piece::King(White), &E1);
        assert!(moves.contains(&Move::KingSideCastle(White)));
    }

    #[test]
    fn knight_on_standard_board() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);

        let e4 = movegen.for_knight(&Piece::Knight(White), &E4);

        assert_eq!(e4.len(), 6);
    }
    #[test]
    fn knight_in_starting_position() {
        let board = Board::new();
        let gen = MoveGen::new(&board);
        let moves = gen.for_knight(&Piece::Knight(White), &G1);
        let coords: Vec<(&Coord, &Coord)> = moves
            .iter()
            .map(|m| match m {
                Move::Piece(from, to) => (from, to),
                _ => unreachable!(),
            })
            .collect();
        let expected_moves = vec![(&G1, &F3), (&G1, &H3)];

        assert_eq!(coords, expected_moves);
    }

    #[test]
    fn rook_on_empty_board() {
        let board = Board::default();
        let movegen = MoveGen::new(&board);

        let moves = movegen.for_rook(&Piece::Rook(White), &E4);

        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn rook_on_standard_board() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_rook(&Piece::Rook(White), &A5);

        let expected_moves: Vec<Move> = [A6, A7, B5, C5, D5, E5, F5, G5, H5, A4, A3]
            .iter()
            .map(|dest| Move::Piece(A5, *dest))
            .collect();

        assert!(moves.iter().all(|e| expected_moves.contains(e)));
    }

    #[test]
    fn starting_rook() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_rook(&Piece::Rook(White), &A1);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn infiltrated_starting_rook() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_rook(&Piece::Rook(Black), &A1);
        assert_eq!(vec![Move::Piece(A1, A2), Move::Piece(A1, B1)], moves);
    }

    #[test]
    fn bishop_on_empty() {
        let board = Board::default();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_bishop(&Piece::Bishop(White), &E5);

        let expected_moves: Vec<Move> = [D4, C3, B2, A1, F6, G7, H8, F4, G3, H2, D6, C7, B8]
            .iter()
            .map(|dest| Move::Piece(E5, *dest))
            .collect();

        assert!(moves.iter().all(|e| expected_moves.contains(e)));
    }

    #[test]
    fn bishop_on_standard() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_bishop(&Piece::Bishop(White), &E5);

        let expected_moves: Vec<Move> = [D4, C3, F6, G7, F4, G3, D6, C7]
            .iter()
            .map(|dest| Move::Piece(E5, *dest))
            .collect();

        assert!(moves.iter().all(|e| expected_moves.contains(e)));
    }

    #[test]
    fn white_pawn_at_start() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_pawn(&Piece::Pawn(White), &A2);
        let expected_moves: Vec<Move> = [A3, A4]
            .into_iter()
            .map(|dest| Move::Piece(A2, dest))
            .collect();
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn pawn_capture() {
        let mut board = Board::default();
        board.set_piece_at(Some(Piece::Pawn(Black)), D3);
        board.set_piece_at(Some(Piece::Bishop(White)), C3);
        board.set_piece_at(Some(Piece::Rook(White)), E3);
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_pawn(&Piece::Pawn(Black), &D4);
        let expected_moves: Vec<Move> = [C3, E3]
            .into_iter()
            .map(|dest| Move::Piece(D4, dest))
            .collect();
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn black_pawn_at_start_with_capture() {
        let mut board = Board::default();

        board.set_piece_at(Some(Piece::Bishop(White)), A6);
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_pawn(&Piece::Pawn(Black), &B7);
        let expected_moves: Vec<Move> = [A6, B6, B5]
            .into_iter()
            .map(|dest| Move::Piece(B7, dest))
            .collect();
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn promote_white_pawn() {
        let mut board = Board::default();

        board.set_piece_at(Some(Piece::Bishop(White)), A6);
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_pawn(&Piece::Pawn(Black), &B7);
        let expected_moves: Vec<Move> = [A6, B6, B5]
            .into_iter()
            .map(|dest| Move::Piece(B7, dest))
            .collect();
        assert_eq!(moves, expected_moves);
    }

    fn move_lists_has_all_targets(
        start_pos: Coord,
        expected_targets: &Vec<Coord>,
        moves: &Vec<Move>,
    ) -> bool {
        expected_targets
            .iter()
            .map(|target| Move::Piece(start_pos, *target))
            .all(|m| moves.contains(&m))
    }
}
