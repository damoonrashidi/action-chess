use super::{
    cooldowns::{
        BOARD_TICK_RATE, COOLDOWN_BISHOP, COOLDOWN_KING, COOLDOWN_KNIGHT, COOLDOWN_PAWN,
        COOLDOWN_QUEEN, COOLDOWN_ROOK,
    },
    coordinate::Coord,
    movegen::MoveGen,
    piece::{Color, Move, Piece},
};
use core::fmt;
use std::{collections::HashSet, fmt::Debug, time::Duration};

#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],

    pub white_can_castle_kingside: bool,
    pub white_can_castle_queenside: bool,
    pub black_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,

    pub white_hp: usize,
    pub black_hp: usize,
}

impl Board {
    #[must_use]
    pub fn standard() -> Self {
        let mut pieces: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        pieces[1] = [Some(Piece::Pawn(Color::White, COOLDOWN_PAWN)); 8];
        pieces[6] = [Some(Piece::Pawn(Color::Black, COOLDOWN_PAWN)); 8];

        pieces[0] = [
            Some(Piece::Rook(Color::White, COOLDOWN_ROOK)),
            Some(Piece::Knight(Color::White, COOLDOWN_KNIGHT)),
            Some(Piece::Bishop(Color::White, COOLDOWN_BISHOP)),
            Some(Piece::Queen(Color::White, COOLDOWN_QUEEN)),
            Some(Piece::King(Color::White, COOLDOWN_KING)),
            Some(Piece::Bishop(Color::White, COOLDOWN_BISHOP)),
            Some(Piece::Knight(Color::White, COOLDOWN_KNIGHT)),
            Some(Piece::Rook(Color::White, COOLDOWN_ROOK)),
        ];

        pieces[7] = [
            Some(Piece::Rook(Color::Black, COOLDOWN_ROOK)),
            Some(Piece::Knight(Color::Black, COOLDOWN_KNIGHT)),
            Some(Piece::Bishop(Color::Black, COOLDOWN_BISHOP)),
            Some(Piece::Queen(Color::Black, COOLDOWN_QUEEN)),
            Some(Piece::King(Color::Black, COOLDOWN_KING)),
            Some(Piece::Bishop(Color::Black, COOLDOWN_BISHOP)),
            Some(Piece::Knight(Color::Black, COOLDOWN_KNIGHT)),
            Some(Piece::Rook(Color::Black, COOLDOWN_ROOK)),
        ];

        Self {
            pieces,
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
            white_hp: 5_000,
            black_hp: 5_000,
        }
    }

    #[must_use]
    pub fn empty() -> Self {
        Self {
            pieces: [[None; 8]; 8],
            white_can_castle_kingside: false,
            white_can_castle_queenside: false,
            black_can_castle_kingside: false,
            black_can_castle_queenside: false,
            white_hp: 0,
            black_hp: 0,
        }
    }

    #[must_use]
    pub fn get_piece_count(&self) -> usize {
        self.pieces
            .into_iter()
            .map(|row| row.iter().filter_map(|p| *p).count())
            .sum()
    }

    #[must_use]
    pub fn get_piece_at(&self, position: &Coord) -> &Option<Piece> {
        if !position.is_valid() {
            return &None;
        }
        #[allow(clippy::cast_sign_loss)]
        &self.pieces[position.1 as usize][position.0 as usize]
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn set_piece_at(&mut self, piece: Option<Piece>, position: Coord) {
        self.pieces[position.1 as usize][position.0 as usize] = piece;
    }

    pub fn remove_by_piece(&mut self, piece: &Piece) {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(target) = self.pieces[y][x] {
                    if piece == &target {
                        self.pieces[y][x] = None;
                    }
                }
            }
        }
    }

    #[must_use]
    pub fn filter_by_piece(&self, piece: Piece) -> Board {
        let mut pieces = [[None; 8]; 8];
        (0..8).for_each(|y| {
            (0..8).for_each(|x| {
                pieces[y][x] = match (piece, self.pieces[y][x]) {
                    (_, None) => None,
                    (comp, Some(target)) => {
                        if comp == target {
                            Some(target)
                        } else {
                            None
                        }
                    }
                };
            });
        });

        Board {
            pieces,
            ..self.clone()
        }
    }

    #[must_use]
    pub fn filter_by_color(&self, color: Color) -> Board {
        let mut pieces = [[None; 8]; 8];

        (0..8).for_each(|y| {
            (0..8).for_each(|x| {
                pieces[y][x] = match (color, self.pieces[y][x]) {
                    (_, None) => None,
                    (comp, Some(target)) => {
                        if comp == target.get_color() {
                            Some(target)
                        } else {
                            None
                        }
                    }
                };
            });
        });

        Board {
            pieces,
            ..self.clone()
        }
    }

    pub fn process_move(&mut self, m: Move) -> &Self {
        match m {
            Move::Piece(from, to) => {
                if let Some(mut piece) = self.get_piece_at(&from) {
                    piece.set_cooldown(Piece::std_piece_cooldown(&piece));
                    self.set_piece_at(Some(piece), to);
                }
                self.set_piece_at(None, from);
            }
            Move::KingSideCastle(color) => {
                let src_file = if color == Color::White { 0 } else { 7 };
                if color == Color::White {
                    self.white_can_castle_kingside = false;
                    self.white_can_castle_queenside = false;
                } else {
                    self.black_can_castle_kingside = false;
                    self.black_can_castle_queenside = false;
                }
                self.pieces[src_file][4] = None;
                self.pieces[src_file][7] = None;
                self.pieces[src_file][6] = Some(Piece::King(color, COOLDOWN_KING));
                self.pieces[src_file][5] = Some(Piece::Rook(color, COOLDOWN_ROOK));
            }
            Move::QueenSideCastle(color) => {
                let src_file = if color == Color::White { 0 } else { 7 };
                self.pieces[src_file][4] = None;
                self.pieces[src_file][0] = None;
                self.pieces[src_file][2] = Some(Piece::King(color, COOLDOWN_KING));
                self.pieces[src_file][2] = Some(Piece::Rook(color, COOLDOWN_ROOK));
            }
            #[allow(clippy::cast_sign_loss)]
            Move::Promotion(src, dest, piece) => {
                self.pieces[src.1 as usize][src.0 as usize] = None;
                self.pieces[dest.1 as usize][dest.0 as usize] = Some(piece);
            }
        }

        self
    }

    #[must_use]
    pub fn get_coord_for_piece(&self, piece: &Piece) -> Option<Coord> {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(target) = self.pieces[y][x] {
                    if std::mem::discriminant(&target) == std::mem::discriminant(piece)
                        && target.get_color() == piece.get_color()
                    {
                        #[allow(clippy::cast_possible_truncation)]
                        return Some(Coord(x as i8, y as i8));
                    }
                }
            }
        }
        None
    }

    pub fn tick(&mut self) {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(mut piece) = self.pieces[y][x] {
                    let new_cd = piece.get_cooldown().saturating_sub(BOARD_TICK_RATE);
                    piece.set_cooldown(new_cd);
                    self.pieces[y][x] = Some(piece);
                }
            }
        }

        let (attacks_on_white, attacks_on_black) = self.king_check_count();
        self.white_hp = self.white_hp.saturating_sub(attacks_on_white as usize);
        self.black_hp = self.white_hp.saturating_sub(attacks_on_black as usize);
    }

    #[must_use]
    pub fn king_check_count(&self) -> (u8, u8) {
        let mut attacks_on_white = 0;
        let mut attacks_on_black = 0;

        let gen = MoveGen::new(self);
        let mut attack_positions: HashSet<Coord> = HashSet::new();

        let white_king_pos =
            self.get_coord_for_piece(&Piece::King(Color::White, Duration::default()));
        let black_king_pos =
            self.get_coord_for_piece(&Piece::King(Color::Black, Duration::default()));

        for mv in gen.get_possible_moves() {
            match (mv, white_king_pos) {
                (Move::Piece(_, to), Some(white_king_pos)) => {
                    if to == white_king_pos {
                        attacks_on_white += 1;
                    }
                }
                (Move::Promotion(from, to, _), Some(white_king_pos)) => {
                    if to == white_king_pos && !attack_positions.contains(&from) {
                        attack_positions.insert(from);
                        attacks_on_white += 1;
                    }
                }
                _ => {}
            };
            match (mv, black_king_pos) {
                (Move::Piece(_, to), Some(black_king_pos)) => {
                    if to == black_king_pos {
                        attacks_on_black += 1;
                    }
                }
                (Move::Promotion(from, to, _), Some(black_king_pos)) => {
                    if to == black_king_pos && !attack_positions.contains(&from) {
                        attack_positions.insert(from);
                        attacks_on_black += 1;
                    }
                }
                _ => {}
            };
        }

        (attacks_on_white, attacks_on_black)
    }

    #[must_use]
    pub fn is_valid_move(&self, mv: Move) -> bool {
        MoveGen::new(self).get_possible_moves().contains(&mv)
    }
}

impl From<&str> for Board {
    fn from(fen: &str) -> Board {
        let parts = fen.split_whitespace().collect::<Vec<&str>>();
        let positions = parts.first().expect("{fen} is not a valid FEN");
        let mut pieces: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        let mut file = 0;
        let mut rank = 7;
        for c in positions.chars() {
            if c == '/' {
                file = 0;
                rank -= 1;
            }

            if ('0'..'8').contains(&c) {
                if let Ok(empties) = c.to_string().parse::<usize>() {
                    for _ in 0..empties {
                        pieces[rank][file] = None;
                        file += 1;
                        continue;
                    }
                }
            }
            let piece = match c {
                'p' => Some(Piece::Pawn(Color::Black, COOLDOWN_PAWN)),
                'b' => Some(Piece::Bishop(Color::Black, COOLDOWN_BISHOP)),
                'n' => Some(Piece::Knight(Color::Black, COOLDOWN_KNIGHT)),
                'r' => Some(Piece::Rook(Color::Black, COOLDOWN_ROOK)),
                'q' => Some(Piece::Queen(Color::Black, COOLDOWN_QUEEN)),
                'k' => Some(Piece::King(Color::Black, COOLDOWN_KING)),
                'P' => Some(Piece::Pawn(Color::White, COOLDOWN_PAWN)),
                'B' => Some(Piece::Bishop(Color::White, COOLDOWN_BISHOP)),
                'N' => Some(Piece::Knight(Color::White, COOLDOWN_KNIGHT)),
                'R' => Some(Piece::Rook(Color::White, COOLDOWN_ROOK)),
                'Q' => Some(Piece::Queen(Color::White, COOLDOWN_QUEEN)),
                'K' => Some(Piece::King(Color::White, COOLDOWN_KING)),
                _ => None,
            };
            if file < 8 {
                pieces[rank][file] = piece;
                if piece.is_some() {
                    file += 1;
                }
            }
        }

        let castle_rights = parts.get(2).expect("{fen} is not a valid fen");
        let mut black_kingside = false;
        let mut white_kingside = false;
        let mut black_queenside = false;
        let mut white_queenside = false;

        for c in castle_rights.chars() {
            match c {
                'k' => black_kingside = true,
                'K' => white_kingside = true,
                'q' => black_queenside = true,
                'Q' => white_queenside = true,
                _ => {}
            }
        }

        let white_king_hp = parts.get(6).and_then(|hp| hp.parse().ok()).unwrap_or(1000);
        let black_king_hp = parts.get(7).and_then(|hp| hp.parse().ok()).unwrap_or(1000);

        Board {
            pieces,
            white_can_castle_kingside: white_kingside,
            white_can_castle_queenside: white_queenside,
            black_can_castle_kingside: black_kingside,
            black_can_castle_queenside: black_queenside,
            white_hp: white_king_hp,
            black_hp: black_king_hp,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dash = "\u{02501}";
        let trip_dash = "\u{02501}\u{02501}\u{02501}";
        let pipe = "\u{02503}";

        let mut render = format!("\t\tBlack HP: {}\n\n", self.black_hp);
        render = format!("{render}      A | B | C | D | E | F | G | H \n");
        render = format!("{render}    \u{0250F}{}\u{02513}\n", [dash; 31].join(""));

        for y in (0..8).rev() {
            render = format!("\n{render}  {} ", y + 1);
            for x in 0..8 {
                match self.pieces[y][x] {
                    Some(p) => render = format!("{render}{pipe} {p} "),
                    None => render = format!("{render}{pipe}   "),
                }
            }
            render = format!("{render}{pipe} {}\n ", y + 1);
            if y == 0 {
                render = format!(
                    "{render}   \u{02517}{}\u{0251B}\n",
                    [trip_dash; 8].join("\u{0253B}")
                );
            } else {
                render = format!("{render}   {pipe}{}{pipe}\n", [trip_dash; 8].join("╋"));
            }
        }

        render = format!("{render}      A | B | C | D | E | F | G | H \n");
        render = format!("{render}\n\t\tWhite HP: {}", self.white_hp);

        write!(f, "{render}")
    }
}
