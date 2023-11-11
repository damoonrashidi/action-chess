#[deny(clippy::all, clippy::pedantic)]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub mod board;
pub mod cooldowns;
pub mod coordinate;
pub mod piece;
pub mod square;

mod movegen;
mod tests;
