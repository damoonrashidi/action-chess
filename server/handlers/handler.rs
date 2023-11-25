use std::net::SocketAddr;

use crate::world::World;

pub(crate) trait Handler {
    fn handle(player: SocketAddr, msg: [u8; 4], world: &mut World);
}
