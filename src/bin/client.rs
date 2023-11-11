// use std::net::{SocketAddr, UdpSocket};

// use action_chess::{
//     network::{command::Command, game_command::GameCmd},
//     state::{piece::Move, square::*},
// };

// fn main() -> std::io::Result<()> {
//     let socket = UdpSocket::bind("0.0.0.0:8888")?;
//     let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
//     socket.connect(addr)?;

//     println!("joining game");
//     let join: Command = GameCmd::Join("abc".into()).into();
//     let _ = socket.send(&join);

//     let buf: Command = Move::Piece(A2, A4).into();
//     println!("sending move");
//     let _ = socket.send(&buf);

//     let mut msg: Command = [0; 4];
//     loop {
//         let _res = socket.recv(&mut msg)?;
//         println!("{}", Move::from(msg));
//     }
// }

fn main() {}
