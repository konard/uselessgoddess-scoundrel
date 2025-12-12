#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

use crate::prelude::*;

mod core;
mod game;
pub mod prelude;
mod ui;

/// Main game plugin that sets up all systems and resources.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(core::plugin);
        app.add_plugins(game::plugin);
        app.add_plugins(ui::plugin);
    }
}
