#[deny(clippy::all, clippy::pedantic)]
#[allow(clippy::cast_sign_loss)]
pub mod helpers;

#[allow(clippy::cast_sign_loss)]
#[deny(clippy::all, clippy::pedantic)]
pub mod network;

#[deny(clippy::all, clippy::pedantic)]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub mod state;
