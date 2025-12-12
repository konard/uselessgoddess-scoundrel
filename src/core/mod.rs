use bevy::prelude::*;

mod camera;
mod effects;
mod input;

pub use camera::*;
pub use effects::*;
pub use input::*;

/// Core plugin that sets up camera, effects, and input systems.
pub fn plugin(app: &mut App) {
    app.add_plugins(camera::plugin);
    app.add_plugins(effects::plugin);
    app.add_plugins(input::plugin);
}
