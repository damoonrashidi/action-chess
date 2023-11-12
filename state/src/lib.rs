#[deny(clippy::all, clippy::pedantic, rustdoc::all)]
#[allow(clippy::cast_sign_loss)]
pub mod board;
pub mod cooldowns;
pub mod coordinate;
pub mod encode;
pub mod piece;
pub mod square;

mod movegen;
mod tests;
