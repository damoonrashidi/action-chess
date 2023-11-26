use state::{
    cooldowns::{
        COOLDOWN_BISHOP, COOLDOWN_KING, COOLDOWN_KNIGHT, COOLDOWN_PAWN, COOLDOWN_QUEEN,
        COOLDOWN_ROOK,
    },
    coordinate::Coord,
    piece::{Color, Move, Piece},
};

pub(crate) fn parse_move(move_str: &str) -> Option<Move> {
    let castling = match move_str {
        "o-o" => Some(Move::KingSideCastle(Color::White)),
        "o-o-o" => Some(Move::QueenSideCastle(Color::White)),
        "O-O" => Some(Move::KingSideCastle(Color::Black)),
        "O-O-O" => Some(Move::QueenSideCastle(Color::Black)),
        _ => None,
    };

    if castling.is_some() {
        return castling;
    }

    let bits = move_str.split(' ');

    let parts = bits.fold((None, None, None), |(from, to, _), bit| {
        if from.is_none() {
            return (str_to_coord(bit), None, None);
        } else if to.is_none() {
            return (from, str_to_coord(bit), None);
        }

        (from, to, str_to_piece(bit))
    });

    match parts {
        (Some(from), Some(to), None) => Some(Move::Piece(from, to)),
        (Some(from), Some(to), Some(piece)) => Some(Move::Promotion(from, to, piece)),
        _ => None,
    }
}

pub(crate) fn str_to_coord(str: &str) -> Option<Coord> {
    let mut chars = str.chars();
    let f = chars.next()?;
    let r = chars.next()?;

    let file = match f {
        'a'..='h' => Some(f.to_digit(36)? - 10),
        _ => None,
    }?;

    let rank = match r {
        '1'..='8' => Some(r.to_digit(10)? - 1),
        _ => None,
    }?;

    #[allow(clippy::cast_possible_truncation)]
    Some(Coord(file as i8, rank as i8))
}

pub(crate) fn str_to_piece(str: &str) -> Option<Piece> {
    let color = if str.to_lowercase() == str {
        Color::White
    } else {
        Color::Black
    };

    match str.to_lowercase().trim() {
        "p" => Some(Piece::Pawn(color, COOLDOWN_PAWN)),
        "n" => Some(Piece::Knight(color, COOLDOWN_KNIGHT)),
        "b" => Some(Piece::Bishop(color, COOLDOWN_BISHOP)),
        "r" => Some(Piece::Rook(color, COOLDOWN_ROOK)),
        "q" => Some(Piece::Queen(color, COOLDOWN_QUEEN)),
        "k" => Some(Piece::King(color, COOLDOWN_KING)),
        _ => None,
    }
}
