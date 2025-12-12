use bevy::prelude::*;

/// Marker for the CRT overlay entity.
#[derive(Component)]
pub struct CrtOverlay;

/// Marker for the vignette overlay entity.
#[derive(Component)]
pub struct VignetteOverlay;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_overlays);
    app.add_systems(Update, update_scanline_effect);
}

fn setup_overlays(mut commands: Commands) {
    // Create scanline overlay using sprites (simpler than full shader)
    // This creates a semi-transparent striped pattern for CRT effect
    let num_lines = 100;
    let line_height = 4.0;
    let screen_height = 800.0;

    for i in 0..num_lines {
        let y = -screen_height / 2.0 + (i as f32) * (screen_height / num_lines as f32);
        commands.spawn((
            CrtOverlay,
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.15),
                custom_size: Some(Vec2::new(1400.0, line_height / 2.0)),
                ..default()
            },
            Transform::from_xyz(0.0, y, 100.0),
        ));
    }

    // Vignette effect using corner sprites
    let vignette_color = Color::srgba(0.0, 0.0, 0.0, 0.0);
    let vignette_size = 600.0;

    // Top-left corner gradient
    commands.spawn((
        VignetteOverlay,
        Sprite {
            color: vignette_color,
            custom_size: Some(Vec2::new(vignette_size, vignette_size)),
            ..default()
        },
        Transform::from_xyz(-500.0, 300.0, 99.0),
    ));

    // Top-right corner gradient
    commands.spawn((
        VignetteOverlay,
        Sprite {
            color: vignette_color,
            custom_size: Some(Vec2::new(vignette_size, vignette_size)),
            ..default()
        },
        Transform::from_xyz(500.0, 300.0, 99.0),
    ));

    // Bottom-left corner gradient
    commands.spawn((
        VignetteOverlay,
        Sprite {
            color: vignette_color,
            custom_size: Some(Vec2::new(vignette_size, vignette_size)),
            ..default()
        },
        Transform::from_xyz(-500.0, -300.0, 99.0),
    ));

    // Bottom-right corner gradient
    commands.spawn((
        VignetteOverlay,
        Sprite {
            color: vignette_color,
            custom_size: Some(Vec2::new(vignette_size, vignette_size)),
            ..default()
        },
        Transform::from_xyz(500.0, -300.0, 99.0),
    ));
}

fn update_scanline_effect(time: Res<Time>, mut query: Query<&mut Sprite, With<CrtOverlay>>) {
    // Subtle flicker effect
    let flicker = ((time.elapsed_secs() * 60.0).sin() * 0.5 + 0.5) * 0.05;

    for mut sprite in query.iter_mut() {
        sprite.color = Color::srgba(0.0, 0.0, 0.0, 0.1 + flicker);
    }
}
