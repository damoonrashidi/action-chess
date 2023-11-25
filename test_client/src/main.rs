use std::{
    io::stdin,
    sync::{Arc, Mutex},
    thread::{self},
    time::Duration,
};

use chess_client::ChessClient;
use clap::Parser;
use state::{
    board::Board,
    cooldowns::{
        COOLDOWN_BISHOP, COOLDOWN_KING, COOLDOWN_KNIGHT, COOLDOWN_PAWN, COOLDOWN_QUEEN,
        COOLDOWN_ROOK,
    },
    coordinate::Coord,
    movegen::MoveGen,
    piece::{Color, Move, Piece},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("8000"))]
    port: String,

    #[arg(long)]
    host: String,

    #[arg(short, long)]
    game_id: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let client = Arc::new(ChessClient::new(&args.port, &args.host)?);
    let board = Arc::new(Mutex::new(Board::standard()));
    let move_listener = client.listen();

    client.join_game(&args.game_id);

    println!("Welcome to game {}", args.game_id);
    println!("{}", Board::standard());

    let remote_board = Arc::clone(&board);
    let incoming_commands = thread::spawn(move || {
        for mv in move_listener {
            if let Ok(mut board) = remote_board.lock() {
                board.process_move(mv);
                let gen = MoveGen::new(&board);
                let moves = gen.get_possible_moves_for_color(Color::White);
                MoveGen::render_movelist(&board, &moves);
            }
        }
    });

    let remote_client = Arc::clone(&client);
    let read_moves = thread::spawn(move || -> anyhow::Result<()> {
        let read = stdin();

        loop {
            let mut str = String::new();
            read.read_line(&mut str)?;
            if let Some(mv) = parse_move(&str) {
                remote_client.make_move(mv);
            } else {
                println!(
                    "could not parse {} into move, board state unchanged",
                    str.trim()
                );
            }
        }
    });

    let board_loop = thread::spawn(move || loop {
        if let Ok(mut board) = board.lock() {
            board.tick();
        }
        thread::sleep(Duration::from_millis(16));
    });

    let _ = incoming_commands.join();
    let _ = read_moves.join();
    let _ = board_loop.join();

    Ok(())
}

fn parse_move(move_str: &str) -> Option<Move> {
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

fn str_to_coord(str: &str) -> Option<Coord> {
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

fn str_to_piece(str: &str) -> Option<Piece> {
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
