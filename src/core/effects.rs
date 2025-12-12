use bevy::prelude::*;

use crate::core::PrimaryCamera;

/// Resource to track screen shake state.
#[derive(Resource, Default)]
pub struct ScreenShake {
    pub trauma: f32,
    pub decay: f32,
}

impl ScreenShake {
    pub fn new() -> Self {
        Self {
            trauma: 0.0,
            decay: 5.0,
        }
    }

    /// Add trauma to trigger screen shake. Value should be 0.0-1.0.
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }
}

/// Message to trigger screen shake.
#[derive(Message)]
pub struct ShakeEvent {
    pub intensity: f32,
}

/// Component for entities that can be tweened/lerped.
#[derive(Component)]
pub struct Tween {
    pub start_pos: Vec3,
    pub target_pos: Vec3,
    pub progress: f32,
    pub duration: f32,
    pub easing: EasingFunction,
}

#[derive(Clone, Copy, Default)]
pub enum EasingFunction {
    #[default]
    Linear,
    EaseOutQuad,
    EaseOutCubic,
    EaseOutElastic,
}

impl EasingFunction {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            EasingFunction::EaseOutElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                    2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
        }
    }
}

impl Tween {
    pub fn new(start: Vec3, target: Vec3, duration: f32, easing: EasingFunction) -> Self {
        Self {
            start_pos: start,
            target_pos: target,
            progress: 0.0,
            duration,
            easing,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }
}

pub fn plugin(app: &mut App) {
    app.insert_resource(ScreenShake::new());
    app.add_message::<ShakeEvent>();
    app.add_systems(
        Update,
        (
            handle_shake_events,
            update_screen_shake,
            update_tweens,
            cleanup_completed_tweens,
        ),
    );
}

fn handle_shake_events(mut shake: ResMut<ScreenShake>, mut events: MessageReader<ShakeEvent>) {
    for event in events.read() {
        shake.add_trauma(event.intensity);
    }
}

fn update_screen_shake(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
    mut camera_query: Query<&mut Transform, With<PrimaryCamera>>,
) {
    if shake.trauma <= 0.0 {
        return;
    }

    // Decay trauma over time
    shake.trauma = (shake.trauma - shake.decay * time.delta_secs()).max(0.0);

    // Calculate shake intensity (trauma squared for more dramatic effect)
    let shake_amount = shake.trauma * shake.trauma;

    // Apply random offset to camera
    if let Ok(mut transform) = camera_query.single_mut() {
        let max_offset = 10.0 * shake_amount;
        let offset_x = (rand::random::<f32>() - 0.5) * 2.0 * max_offset;
        let offset_y = (rand::random::<f32>() - 0.5) * 2.0 * max_offset;

        transform.translation.x = offset_x;
        transform.translation.y = offset_y;
    }
}

fn update_tweens(time: Res<Time>, mut query: Query<(&mut Transform, &mut Tween)>) {
    for (mut transform, mut tween) in query.iter_mut() {
        if tween.is_complete() {
            continue;
        }

        tween.progress += time.delta_secs() / tween.duration;
        tween.progress = tween.progress.min(1.0);

        let t = tween.easing.apply(tween.progress);
        transform.translation = tween.start_pos.lerp(tween.target_pos, t);
    }
}

fn cleanup_completed_tweens(mut commands: Commands, query: Query<(Entity, &Tween)>) {
    for (entity, tween) in query.iter() {
        if tween.is_complete() {
            commands.entity(entity).remove::<Tween>();
        }
    }
}
