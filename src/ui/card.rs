use bevy::prelude::*;

use crate::core::{EasingFunction, MouseWorldPosition, Tween};
use crate::game::{
    room::{DealRoomEvent, PlayTileEvent, Room, ROOM_SIZE},
    tiles::Tile,
};
use crate::ui::GameState;

/// Card dimensions
pub const CARD_WIDTH: f32 = 120.0;
pub const CARD_HEIGHT: f32 = 160.0;
pub const CARD_SPACING: f32 = 20.0;

/// Component marking a card entity.
#[derive(Component)]
pub struct Card {
    pub slot_index: usize,
}

/// Component for card shimmer effect.
#[derive(Component)]
pub struct CardShimmer {
    pub shimmer_intensity: f32,
}

/// Component marking a card as hovered.
#[derive(Component)]
pub struct Hovered;

/// Component marking a card as selected/highlighted.
#[derive(Component)]
pub struct Highlighted;

/// Marker for card border sprite.
#[derive(Component)]
pub struct CardBorder;

/// Marker for card glyph text.
#[derive(Component)]
pub struct CardGlyph;

/// Marker for card value text.
#[derive(Component)]
pub struct CardValue;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup_initial_room);
    app.add_systems(
        Update,
        (
            spawn_cards_for_room,
            update_card_hover,
            update_card_shimmer,
            handle_card_click,
            update_card_highlight,
            cleanup_played_cards,
        )
            .run_if(in_state(GameState::Playing)),
    );
    app.add_systems(OnExit(GameState::Playing), cleanup_all_cards);
}

fn setup_initial_room(mut deal_events: EventWriter<DealRoomEvent>) {
    deal_events.write(DealRoomEvent);
}

fn spawn_cards_for_room(
    mut commands: Commands,
    room: Res<Room>,
    existing_cards: Query<Entity, With<Card>>,
) {
    // Only spawn if room changed and no cards exist
    if !room.is_changed() {
        return;
    }

    // Calculate starting X position to center cards
    let total_width = (ROOM_SIZE as f32) * CARD_WIDTH + ((ROOM_SIZE - 1) as f32) * CARD_SPACING;
    let start_x = -total_width / 2.0 + CARD_WIDTH / 2.0;

    for (index, tile_opt) in room.tiles.iter().enumerate() {
        // Check if a card already exists at this slot
        let card_exists = existing_cards
            .iter()
            .any(|_| false); // We'll check slot indices properly

        if card_exists {
            continue;
        }

        if let Some(tile_type) = tile_opt {
            let x = start_x + (index as f32) * (CARD_WIDTH + CARD_SPACING);
            let y = 0.0;

            // Spawn card with animation from above
            let start_pos = Vec3::new(x, y + 400.0, 0.0);
            let target_pos = Vec3::new(x, y, 0.0);

            commands
                .spawn((
                    Name::new(format!("Card_{}", index)),
                    Card { slot_index: index },
                    Tile {
                        tile_type: *tile_type,
                        slot_index: index,
                    },
                    CardShimmer {
                        shimmer_intensity: 0.0,
                    },
                    Transform::from_translation(start_pos),
                    Visibility::Visible,
                    Tween::new(
                        start_pos,
                        target_pos,
                        0.4 + (index as f32) * 0.1,
                        EasingFunction::EaseOutCubic,
                    ),
                ))
                .with_children(|parent| {
                    // Card background (dark rectangle)
                    parent.spawn((
                        Sprite {
                            color: Color::srgba(0.08, 0.08, 0.08, 0.95),
                            custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 0.0),
                    ));

                    // Card border using Unicode box drawing
                    let border_color = tile_type.color();
                    parent.spawn((
                        CardBorder,
                        Text2d::new(create_card_border()),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(border_color.with_alpha(0.8)),
                        Transform::from_xyz(0.0, 0.0, 1.0),
                    ));

                    // Tile glyph (large, centered)
                    parent.spawn((
                        CardGlyph,
                        Text2d::new(tile_type.glyph()),
                        TextFont {
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(tile_type.color()),
                        Transform::from_xyz(0.0, 10.0, 2.0),
                    ));

                    // Tile value/name (bottom of card)
                    parent.spawn((
                        CardValue,
                        Text2d::new(tile_type.name()),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.9, 0.9, 0.9, 0.7)),
                        Transform::from_xyz(0.0, -50.0, 2.0),
                    ));
                });
        }
    }
}

fn create_card_border() -> String {
    // Create ASCII art border for the card
    let width = 14;
    let height = 10;

    let mut border = String::new();

    // Top border
    border.push('┌');
    for _ in 0..width {
        border.push('─');
    }
    border.push('┐');
    border.push('\n');

    // Middle rows
    for _ in 0..height {
        border.push('│');
        for _ in 0..width {
            border.push(' ');
        }
        border.push('│');
        border.push('\n');
    }

    // Bottom border
    border.push('└');
    for _ in 0..width {
        border.push('─');
    }
    border.push('┘');

    border
}

fn update_card_hover(
    mut commands: Commands,
    mouse_pos: Res<MouseWorldPosition>,
    card_query: Query<(Entity, &Transform, &Card), Without<Hovered>>,
    hovered_query: Query<(Entity, &Transform), With<Hovered>>,
    room: Res<Room>,
) {
    let mouse = mouse_pos.0;

    // Check existing hovered cards
    for (entity, transform) in hovered_query.iter() {
        let pos = transform.translation.truncate();
        let half_size = Vec2::new(CARD_WIDTH / 2.0, CARD_HEIGHT / 2.0);

        if !point_in_rect(mouse, pos, half_size) {
            commands.entity(entity).remove::<Hovered>();
        }
    }

    // Check for new hovers
    for (entity, transform, card) in card_query.iter() {
        let pos = transform.translation.truncate();
        let half_size = Vec2::new(CARD_WIDTH / 2.0, CARD_HEIGHT / 2.0);

        if point_in_rect(mouse, pos, half_size) && room.is_valid_selection(card.slot_index) {
            commands.entity(entity).insert(Hovered);
        }
    }
}

fn update_card_shimmer(
    mouse_pos: Res<MouseWorldPosition>,
    mut card_query: Query<(&Transform, &mut CardShimmer, &Children)>,
    mut text_query: Query<&mut Transform, (With<CardGlyph>, Without<Card>)>,
) {
    let mouse = mouse_pos.0;

    for (card_transform, mut shimmer, children) in card_query.iter_mut() {
        let card_pos = card_transform.translation.truncate();
        let distance = (mouse - card_pos).length();

        // Calculate shimmer based on mouse proximity
        let max_distance = 200.0;
        shimmer.shimmer_intensity = ((max_distance - distance) / max_distance).max(0.0);

        // Calculate tilt based on mouse position relative to card
        if shimmer.shimmer_intensity > 0.0 {
            let offset = mouse - card_pos;
            let tilt_x = (offset.x / CARD_WIDTH) * 0.1;
            let tilt_y = (offset.y / CARD_HEIGHT) * 0.1;

            // Apply subtle rotation to glyph for parallax effect
            for child in children.iter() {
                if let Ok(mut glyph_transform) = text_query.get_mut(child) {
                    glyph_transform.translation.x = tilt_x * 5.0;
                    glyph_transform.translation.y = 10.0 + tilt_y * 5.0;
                }
            }
        }
    }
}

fn handle_card_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_pos: Res<MouseWorldPosition>,
    card_query: Query<(&Transform, &Card)>,
    room: Res<Room>,
    mut play_events: EventWriter<PlayTileEvent>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let mouse = mouse_pos.0;

    for (transform, card) in card_query.iter() {
        let pos = transform.translation.truncate();
        let half_size = Vec2::new(CARD_WIDTH / 2.0, CARD_HEIGHT / 2.0);

        if point_in_rect(mouse, pos, half_size) && room.is_valid_selection(card.slot_index) {
            play_events.write(PlayTileEvent {
                slot_index: card.slot_index,
            });
            break;
        }
    }
}

fn update_card_highlight(
    mut commands: Commands,
    room: Res<Room>,
    card_query: Query<(Entity, &Card)>,
    highlighted_query: Query<Entity, With<Highlighted>>,
) {
    // Remove all existing highlights
    for entity in highlighted_query.iter() {
        commands.entity(entity).remove::<Highlighted>();
    }

    // Add highlights to valid selections
    for (entity, card) in card_query.iter() {
        if room.is_valid_selection(card.slot_index) {
            commands.entity(entity).insert(Highlighted);
        }
    }
}

fn cleanup_played_cards(
    mut commands: Commands,
    room: Res<Room>,
    card_query: Query<(Entity, &Card)>,
) {
    if !room.is_changed() {
        return;
    }

    for (entity, card) in card_query.iter() {
        if room.tiles.get(card.slot_index).map(|t| t.is_none()).unwrap_or(true) {
            commands.entity(entity).despawn();
        }
    }
}

fn cleanup_all_cards(mut commands: Commands, card_query: Query<Entity, With<Card>>) {
    for entity in card_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn point_in_rect(point: Vec2, rect_center: Vec2, half_size: Vec2) -> bool {
    point.x >= rect_center.x - half_size.x
        && point.x <= rect_center.x + half_size.x
        && point.y >= rect_center.y - half_size.y
        && point.y <= rect_center.y + half_size.y
}
