use std::net::{SocketAddr, UdpSocket};

use action_chess::{
    network::{command::Command, game_command::GameCmd},
    state::{
        piece::Move,
        square::{A3, D5},
    },
};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8888")?;
    println!("udp socket connection set up");
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    socket.connect(addr)?;

    let buf: Command = Move::Piece(A3, D5).into();
    let join: Command = GameCmd::Join(34524).into();
    let _ = socket.send(&join);
    let _ = socket.send(&buf);

    let mut msg: Command = [0; 4];
    loop {
        let _res = socket.recv(&mut msg)?;
        println!("{}", Move::from(msg));
    }
}
