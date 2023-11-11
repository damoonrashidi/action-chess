#[allow(clippy::cast_sign_loss)]
#[deny(clippy::all, clippy::pedantic)]
pub mod command;
pub mod game_command;
pub mod marshal;
pub mod unmarshal;

mod constants;
mod tests;
