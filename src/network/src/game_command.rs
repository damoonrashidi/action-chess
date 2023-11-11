use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum GameCmd {
    Join(String),
    Leave,
    Resign,
}

impl Display for GameCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GameCmd::Join(game_id) => {
                format!("Join {game_id}")
            }
            GameCmd::Leave => "Leave".into(),
            GameCmd::Resign => "Resign".into(),
        };

        write!(f, "{s}")
    }
}
