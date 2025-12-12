use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
};

/// Marker component for the primary game camera.
#[derive(Component)]
pub struct PrimaryCamera;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        PrimaryCamera,
        Camera2d,
        Camera {
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::srgb(
                0x10 as f32 / 255.0,
                0x10 as f32 / 255.0,
                0x10 as f32 / 255.0,
            )),
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.15,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            ..default()
        },
    ));
}
