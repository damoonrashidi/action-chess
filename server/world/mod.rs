pub mod game;

use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

use game::Game;

#[derive(Debug)]
pub struct World {
    pub socket: UdpSocket,
    games: HashMap<String, Game>,
    participants: HashMap<SocketAddr, String>,
}

impl World {
    #[must_use]
    pub fn new(socket: UdpSocket) -> Self {
        Self {
            games: HashMap::new(),
            participants: HashMap::new(),
            socket,
        }
    }

    pub fn games_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, String, Game> {
        self.games.values_mut()
    }

    #[must_use]
    pub fn get_game(&self, game_id: &String) -> Option<&Game> {
        self.games.get(game_id)
    }

    pub fn get_game_for_player_mut(&mut self, player: &SocketAddr) -> Option<&mut Game> {
        let game_id = self.participants.get(player)?;
        let game = self.games.get_mut(game_id)?;

        Some(game)
    }

    pub fn create_game(&mut self, game_id: &str) {
        self.games.insert(game_id.into(), Game::new());
    }

    pub fn add_player(&mut self, player: SocketAddr, game_id: &String) -> Option<()> {
        let game = self.games.get_mut(game_id)?;
        game.add_player(player);
        self.participants.insert(player, game_id.to_string());

        None
    }
}
