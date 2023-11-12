use std::{collections::HashMap, net::SocketAddr};

use super::game::Game;

#[derive(Default)]
pub struct WorldState {
    games: HashMap<String, Game>,
    participants: HashMap<SocketAddr, String>,
}

impl WorldState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
            participants: HashMap::new(),
        }
    }

    pub fn get_game_mut(&mut self, game_id: &String) -> Option<&mut Game> {
        self.games.get_mut(game_id)
    }

    pub fn get_game_for_player_mut(&mut self, player: &SocketAddr) -> Option<&mut Game> {
        let game_id = self.participants.get(player)?;
        let game = self.games.get_mut(game_id)?;

        Some(game)
    }

    pub fn add_player(&mut self, player: SocketAddr, game_id: &String) -> Option<()> {
        let game = self.games.get_mut(game_id)?;
        game.add_player(player);

        None
    }
}
