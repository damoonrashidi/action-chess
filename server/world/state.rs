use std::{collections::HashMap, net::SocketAddr};

use super::game::Game;
#[allow(unused)]
#[derive(Default, Debug)]
pub struct State {
    games: HashMap<String, Game>,
    participants: HashMap<SocketAddr, String>,
}

#[allow(unused)]
impl State {
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

    pub fn create_game(&mut self, game_id: &str) {
        self.games.insert(game_id.into(), Game::new());
    }

    pub fn add_player(&mut self, player: SocketAddr, game_id: &String) -> Option<()> {
        let game = self.games.get_mut(game_id)?;
        game.add_player(player);

        None
    }
}
