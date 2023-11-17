use std::{
    net::UdpSocket,
    sync::mpsc::{self, Receiver},
    thread,
};

use network::{command::Command, game_command::GameCmd, marshal::Marshal, unmarshal::Unmarshal};
use state::piece::Move;

#[derive(Debug)]
pub struct ChessClient {
    connection: UdpSocket,
}

impl ChessClient {
    /// # Errors
    ///
    /// This function will return an error if you cannot bind to the host UDP socket.
    pub fn new(host: &'static str) -> std::io::Result<Self> {
        let connection = UdpSocket::bind("127.0.0.1:8000")?;
        let _ = connection.connect(host);
        Ok(Self { connection })
    }

    /// # Panics
    ///
    /// This function will panic if a listening connnection cannot be reliably be established
    #[must_use]
    pub fn listen(&self) -> Receiver<Move> {
        let conn = self.connection.try_clone().expect("cannot clone socket");
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || -> std::io::Result<()> {
            loop {
                let mut msg = [0; 4];
                conn.recv_from(&mut msg)?;
                let cmd = Unmarshal::command(msg);
                let _ = tx.send(cmd);
            }
        });

        rx
    }

    pub fn make_move(&self, mv: Move) {
        let command: Command = Marshal::command(mv);
        let _ = self.connection.send(&command);
    }

    pub fn join_game(&self, game_id: &str) {
        let cmd = Marshal::game_command(GameCmd::Join(game_id.into()));
        let _ = self.connection.send(&cmd);
    }
}
