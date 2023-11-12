#[deny(clippy::all, clippy::pedantic, rustdoc::all)]
pub mod command;
pub mod game_command;
pub mod marshal;
pub mod unmarshal;

mod constants;
mod tests;
