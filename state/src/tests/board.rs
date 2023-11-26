#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        board::Board,
        cooldowns::{COOLDOWN_KING, COOLDOWN_PAWN, COOLDOWN_QUEEN, COOLDOWN_ROOK},
        piece::{
            Color::{self, Black, White},
            Piece,
        },
        square::{self, A1, A8, H8},
    };

    #[test]
    fn get_by_color() {
        let board = Board::standard();
        let black_pieces = board.filter_by_color(Black);
        let mut count = 0;
        for y in 0..8 {
            for x in 0..8 {
                if black_pieces.pieces[y][x].is_some() {
                    count += 1;
                }
            }
        }
        assert_eq!(count, 16);
    }

    #[test]
    fn get_all_black_pawns() {
        let board = Board::standard();
        let black_pawns = board.filter_by_piece(Piece::Pawn(Black, COOLDOWN_PAWN));
        assert_eq!(black_pawns.pieces.len(), 8);
    }

    #[test]
    fn get_number_of_checks_on_white_king() {
        let mut board = Board::empty();
        board.set_piece_at(Some(Piece::Rook(Color::Black, Duration::ZERO)), A8);
        board.set_piece_at(Some(Piece::Bishop(Color::Black, Duration::ZERO)), H8);
        board.set_piece_at(Some(Piece::King(Color::White, Duration::ZERO)), A1);
        let count = board.king_check_count(Color::White);
        assert_eq!(count, 2);
    }

    #[test]
    fn white_king_health_loss() {
        let mut board = Board::empty();
        board.white_king_hp = 1000;
        board.set_piece_at(Some(Piece::Rook(Color::Black, Duration::ZERO)), A8);
        board.set_piece_at(Some(Piece::Bishop(Color::Black, Duration::ZERO)), H8);
        board.set_piece_at(Some(Piece::King(Color::White, Duration::ZERO)), A1);

        board.tick();

        assert_eq!(board.white_king_hp, 968);
    }

    #[test]
    fn white_king_defeat() {
        let mut board = Board::empty();
        board.white_king_hp = 3;
        board.set_piece_at(Some(Piece::Rook(Color::Black, Duration::ZERO)), A8);
        board.set_piece_at(Some(Piece::Bishop(Color::Black, Duration::ZERO)), H8);
        board.set_piece_at(Some(Piece::King(Color::White, Duration::ZERO)), A1);

        board.tick();

        assert_eq!(board.white_king_hp, 0);
    }

    #[test]
    fn standard_fen() {
        let board = Board::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w qkQK - 0 1");
        let standard = Board::standard();
        assert_eq!(board.pieces, standard.pieces);
        assert!(
            board.white_can_castle_kingside
                && board.white_can_castle_queenside
                && board.black_can_castle_kingside
                && board.black_can_castle_queenside
        );
    }

    #[test]
    fn custom_fen() {
        let board = Board::from("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w qQ - 0 0");
        assert_eq!(
            board.get_piece_at(&square::A8),
            &Some(Piece::Rook(Black, COOLDOWN_ROOK))
        );
        assert_eq!(
            board.get_piece_at(&square::A8),
            &Some(Piece::Rook(Black, COOLDOWN_ROOK))
        );
        assert_eq!(
            board.get_piece_at(&square::A1),
            &Some(Piece::Queen(Black, COOLDOWN_QUEEN))
        );
        assert_eq!(
            board.get_piece_at(&square::E2),
            &Some(Piece::King(White, COOLDOWN_KING))
        );
        assert!(!board.white_can_castle_kingside);
        assert!(board.white_can_castle_queenside);
        assert!(!board.black_can_castle_kingside);
        assert!(board.black_can_castle_queenside);
    }

    #[test]
    fn hps_from_fen() {
        let board =
            Board::from("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w qQ - 0 0 400 1200");
        assert_eq!(board.white_king_hp, 400);
        assert_eq!(board.black_king_hp, 1200);
    }
}
