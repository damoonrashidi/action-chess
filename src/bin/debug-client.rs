use std::{io::Write, net::TcpStream};

use action_chess::{
    network::command::Command,
    state::{
        cooldowns::COOLDOWN_QUEEN,
        piece::{Color, Move, Piece},
        square::*,
    },
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let commands = [
        Move::Piece(E4, E5),
        Move::Promotion(D7, D8, Piece::Queen(Color::Black, COOLDOWN_QUEEN)),
        Move::KingSideCastle(Color::White),
        Move::QueenSideCastle(Color::Black),
    ]
    .into_iter()
    .map(|mv| mv.into())
    .collect::<Vec<Command>>();

    for cmd in commands {
        stream.write_all(&cmd.0)?;
    }

    Ok(())
}
