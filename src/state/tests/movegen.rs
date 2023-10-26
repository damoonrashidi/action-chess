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
        let moves = gen.for_king(&Piece::King(White), &E5);

        assert_eq!(moves.len(), 8);

        assert!(move_lists_has_all_targets(
            E5,
            &vec![D4, D5, D6, E4, E6, F4, F5, F6],
            &moves
        ));
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
        let mut board = Board::new();
        board.set_piece_at(Some(Piece::King(White)), E5);
        let gen = MoveGen::new(&board);
        let moves = gen.for_king(&Piece::King(White), &E5);
        let expected_moves = vec![D5, F5, D4, E4, F4];

        assert!(move_lists_has_all_targets(E5, &expected_moves, &moves));
        assert_eq!(expected_moves.len(), moves.len());
    }

    #[test]
    fn king_cant_move_into_opposing_king() {
        let mut board = Board::default();
        board.set_piece_at(Some(Piece::King(Black)), D5);
        let gen = MoveGen::new(&board);
        let moves = gen.for_king(&Piece::King(White), &D3);
        let expected_moves = vec![C3, E3, C2, D2, E2];
        assert_eq!(moves.len(), expected_moves.len());
        assert!(move_lists_has_all_targets(D3, &expected_moves, &moves));
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
    fn blocked_queenside_castle() {
        let mut board = Board::default();
        board.set_piece_at(Some(Piece::Queen(White)), D1);
        board.set_piece_at(Some(Piece::Rook(White)), A1);
        board.set_piece_at(Some(Piece::Rook(White)), H1);
        board.white_can_castle_kingside = true;
        board.white_can_castle_queenside = true;
        let gen = MoveGen::new(&board);
        let moves = gen.for_king(&Piece::King(White), &E1);
        assert!(moves.contains(&Move::KingSideCastle(White)));
        assert!(!moves.contains(&Move::QueenSideCastle(White)));
    }

    #[test]
    fn moved_king_castle_kingside() {
        let mut board = Board::default();
        board.set_piece_at(Some(Piece::Rook(White)), H1);
        board.white_can_castle_kingside = true;
        let gen = MoveGen::new(&board);
        let moves = gen.for_king(&Piece::King(White), &E2);
        assert!(!moves.contains(&Move::KingSideCastle(White)));
    }

    #[test]
    fn king_castle_queenside() {
        let mut board = Board::default();
        board.set_piece_at(Some(Piece::Rook(White)), A1);
        board.white_can_castle_queenside = true;
        let gen = MoveGen::new(&board);
        let moves = gen.for_king(&Piece::King(White), &E1);
        assert!(moves.contains(&Move::QueenSideCastle(White)));
    }

    #[test]
    fn pinned_pawn() {
        let board = Board::from_fen("8/8/KP5r/8/8/8/8/8 b - - 0 0").unwrap();
        let gen = MoveGen::new(&board);
        let moves = gen.for_pawn(&Piece::Pawn(White), &B6);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn pinned_rook() {
        let board = Board::from_fen("8/8/KR5r/8/8/8/8/8 b - - 0 0").unwrap();
        let gen = MoveGen::new(&board);
        let moves = gen.for_rook(&Piece::Pawn(White), &B6);
        println!("{board}");
        MoveGen::render_movelist(&board, &moves);
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn knight_on_standard_board() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_knight(&Piece::Knight(White), &E4);

        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn knight_in_starting_position() {
        let board = Board::new();
        let gen = MoveGen::new(&board);
        let moves = gen.for_knight(&Piece::Knight(White), &G1);
        let expected_moves = vec![H3, F3];
        MoveGen::render_movelist(&board, &moves);

        assert!(move_lists_has_all_targets(G1, &expected_moves, &moves));
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
        let expected_moves = vec![A6, A7, B5, C5, D5, E5, F5, G5, H5, A4, A3];

        assert!(move_lists_has_all_targets(A5, &expected_moves, &moves));
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
        let expected_moves = vec![D4, C3, B2, A1, F6, G7, H8, F4, G3, H2, D6, C7, B8];

        assert!(move_lists_has_all_targets(E5, &expected_moves, &moves));
    }

    #[test]
    fn bishop_on_standard() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_bishop(&Piece::Bishop(White), &E5);
        let expected_moves = vec![D4, C3, F6, G7, F4, G3, D6, C7];

        assert!(move_lists_has_all_targets(E5, &expected_moves, &moves));
    }

    #[test]
    fn all_black_starting_moves() {
        let board = Board::new().filter_by_color(Black);
        let gen = MoveGen::new(&board);
        let moves = gen.get_possible_moves();
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn all_black_starting_moves_with_capture() {
        let mut board = Board::new().filter_by_color(Black);
        board.set_piece_at(Some(Piece::Pawn(White)), D6);
        let gen = MoveGen::new(&board);
        let moves = gen.get_possible_moves();

        assert_eq!(moves.len(), 22);
    }

    #[test]
    fn white_pawn_at_start() {
        let board = Board::new();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_pawn(&Piece::Pawn(White), &A2);
        let expected_moves = vec![A3, A4];

        assert!(move_lists_has_all_targets(A2, &expected_moves, &moves));
    }

    #[test]
    fn black_pawn_at_start() {
        let mut board = Board::new();
        board.set_piece_at(Some(Piece::Bishop(White)), D5);
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_pawn(&Piece::Pawn(Black), &D7);
        let expected_moves = vec![D6];

        assert!(move_lists_has_all_targets(D7, &expected_moves, &moves));
    }

    #[test]
    fn pawn_capture() {
        let mut board = Board::default();
        board.set_piece_at(Some(Piece::Pawn(Black)), D3);
        board.set_piece_at(Some(Piece::Bishop(White)), C3);
        board.set_piece_at(Some(Piece::Rook(White)), E3);
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_pawn(&Piece::Pawn(Black), &D4);
        let expected_moves = vec![C3, E3];

        assert!(move_lists_has_all_targets(D4, &expected_moves, &moves));
    }

    #[test]
    fn black_pawn_at_start_with_capture() {
        let mut board = Board::default();
        board.set_piece_at(Some(Piece::Bishop(White)), A6);
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_pawn(&Piece::Pawn(Black), &B7);
        let expected_moves = vec![A6, B6, B5];

        assert!(move_lists_has_all_targets(B7, &expected_moves, &moves));
    }

    #[test]
    fn promote_white_pawn() {
        let board = Board::default();
        let movegen = MoveGen::new(&board);
        let moves = movegen.for_pawn(&Piece::Pawn(Black), &B2);
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
