use bevy::prelude::*;

pub mod combat;
pub mod deck;
pub mod room;
pub mod tiles;

/// Game logic plugin that sets up all game systems.
pub fn plugin(app: &mut App) {
    app.add_plugins(tiles::plugin);
    app.add_plugins(deck::plugin);
    app.add_plugins(combat::plugin);
    app.add_plugins(room::plugin);
}
