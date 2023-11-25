use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

use crate::world::state::State;

pub(crate) trait Handler {
    fn handle(player: SocketAddr, msg: [u8; 4], world: &Arc<Mutex<State>>, socket: &UdpSocket);
}
