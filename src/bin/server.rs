use std::{
    collections::HashMap,
    io::Result,
    net::{SocketAddr, UdpSocket},
    thread,
};

use action_chess::{
    network::game_command::GameCmd,
    state::{board::Board, piece::Move},
};

#[allow(unused)]
struct Game {
    id: String,
    participants: Vec<SocketAddr>,
    board: Board,
}

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    let _games: HashMap<String, Game> = HashMap::new();
    let _participants: HashMap<SocketAddr, String> = HashMap::new();

    let thread = thread::spawn(move || -> Result<()> {
        loop {
            let mut msg: [u8; 4] = [0, 0, 0, 0];

            let (_n, addr) = socket.recv_from(&mut msg)?;
            match msg[0] {
                0..=3 => {
                    let mv: Move = msg.into();
                    let data: [u8; 4] = mv.into();
                    let _ = socket.send_to(&data, addr);
                    println!("received {mv} from {addr}");
                }
                _ => {
                    let cmd: GameCmd = msg.into();
                    match cmd {
                        GameCmd::Join(game_id) => println!("joining {game_id}"),
                        GameCmd::Leave => println!("leaving"),
                        GameCmd::Resign => println!("resigning"),
                    };
                }
            }
        }
    });

    let _ = thread.join();

    Ok(())
}
