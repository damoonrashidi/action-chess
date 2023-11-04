use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use action_chess::{network::command::Command, state::piece::Move};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut command = Command([0; 4]);
    for _ in 0..4 {
        match stream.read(&mut command.0) {
            Ok(_) => {
                let mv: Move = command.into();
                println!("message {mv}");
            }
            Err(err) => println!("error: {err:?}"),
        };
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
