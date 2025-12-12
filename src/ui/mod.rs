use bevy::prelude::*;

mod card;
mod hud;
mod menus;
mod overlay;

/// Game state machine.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

/// UI plugin that sets up all UI systems.
pub fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.add_plugins(card::plugin);
    app.add_plugins(hud::plugin);
    app.add_plugins(menus::plugin);
    app.add_plugins(overlay::plugin);
}
