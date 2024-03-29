pub(crate) mod game;

use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

use game::Game;

#[derive(Debug)]
pub(crate) struct World {
    pub(crate) socket: UdpSocket,
    games: HashMap<String, Game>,
    participants: HashMap<SocketAddr, String>,
}

impl World {
    #[must_use]
    pub(crate) fn new(socket: UdpSocket) -> Self {
        Self {
            games: HashMap::new(),
            participants: HashMap::new(),
            socket,
        }
    }

    pub(crate) fn games_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, String, Game> {
        self.games.values_mut()
    }

    pub(crate) fn get_game(&self, game_id: &String) -> Option<&Game> {
        self.games.get(game_id)
    }

    pub(crate) fn get_game_for_player_mut(&mut self, player: &SocketAddr) -> Option<&mut Game> {
        let game_id = self.participants.get(player)?;
        let game = self.games.get_mut(game_id)?;

        Some(game)
    }

    pub(crate) fn create_game(&mut self, game_id: &str) {
        self.games.insert(game_id.into(), Game::new());
    }

    pub(crate) fn add_player(&mut self, player: SocketAddr, game_id: &String) -> Option<()> {
        let game = self.games.get_mut(game_id)?;
        game.add_player(player);
        self.participants.insert(player, game_id.to_string());

        None
    }
}
