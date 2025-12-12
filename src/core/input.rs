use bevy::prelude::*;

/// Resource to track the current mouse position in world coordinates.
#[derive(Resource, Default)]
pub struct MouseWorldPosition(pub Vec2);

/// Resource to track the current mouse position in screen coordinates.
#[derive(Resource, Default)]
pub struct MouseScreenPosition(pub Vec2);

pub fn plugin(app: &mut App) {
    app.insert_resource(MouseWorldPosition::default());
    app.insert_resource(MouseScreenPosition::default());
    app.add_systems(Update, update_mouse_position);
}

fn update_mouse_position(
    mut mouse_world: ResMut<MouseWorldPosition>,
    mut mouse_screen: ResMut<MouseScreenPosition>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    if let Some(cursor_pos) = window.cursor_position() {
        mouse_screen.0 = cursor_pos;

        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            mouse_world.0 = world_pos;
        }
    }
}
