use std::{
    io::Result,
    net::{SocketAddr, UdpSocket},
};

use network::{game_command::GameCmd, marshal::Marshal, unmarshal::Unmarshal};

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8000")?;
    let server_addr: SocketAddr = "127.0.0.1:8080"
        .parse()
        .expect("could not parse remote addr");

    socket.connect(server_addr)?;

    socket.send(&Marshal::game_command(GameCmd::Join("4444".into())))?;

    loop {
        println!("listening for move");
        let mut msg = [0u8, 0, 0, 0];

        socket.recv(&mut msg)?;

        let mv = Unmarshal::command(msg);
        println!("got {mv}");
    }
}
