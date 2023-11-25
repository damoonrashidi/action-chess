mod handlers;
mod world;
use std::{
    io::Result,
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use handlers::handler::Handler;
use world::state::State;

fn main() -> anyhow::Result<()> {
    let world = Arc::new(Mutex::new(State::new()));

    let tick_handle = tick_world(Arc::clone(&world));

    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    let command_handle = handle_commands(Arc::clone(&world), socket);

    let _ = command_handle.join();
    let _ = tick_handle.join();

    Ok(())
}

fn tick_world(world: Arc<Mutex<State>>) -> JoinHandle<()> {
    thread::spawn(move || loop {
        if let Ok(mut world) = world.lock() {
            for game in world.games_mut() {
                game.tick();
            }
        }
        thread::sleep(Duration::from_millis(16));
    })
}

fn handle_commands(world: Arc<Mutex<State>>, socket: UdpSocket) -> JoinHandle<Result<()>> {
    thread::spawn(move || -> Result<()> {
        loop {
            let mut msg: [u8; 4] = [0, 0, 0, 0];
            let (_, addr) = socket.recv_from(&mut msg)?;

            match msg[0] {
                0..=3 => handlers::moves::MoveHandler::handle(addr, msg, &world, &socket),
                _ => handlers::commands::CommandHandler::handle(addr, msg, &world, &socket),
            };
        }
    })
}
