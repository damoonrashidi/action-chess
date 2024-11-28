pub mod handlers;
pub mod world;
use std::{
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use handlers::handler::Handler;
use world::World;

fn main() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    let socket_clone = socket.try_clone()?;

    let world = Arc::new(Mutex::new(World::new(socket)));
    let command_handle = handle_commands(&world, socket_clone);

    let _ = command_handle.join();

    Ok(())
}

fn handle_commands(
    world: &Arc<Mutex<World>>,
    socket: UdpSocket,
) -> JoinHandle<std::io::Result<()>> {
    let world = Arc::clone(world);
    thread::spawn(move || -> std::io::Result<()> {
        loop {
            let mut msg: [u8; 4] = [0, 0, 0, 0];
            let (_, addr) = socket.recv_from(&mut msg)?;

            if let Ok(mut world) = world.lock() {
                match msg[0] {
                    0..=3 => handlers::moves::MoveHandler::handle(addr, msg, &mut world),
                    _ => handlers::commands::CommandHandler::handle(addr, msg, &mut world),
                };
            }
        }
    })
}
