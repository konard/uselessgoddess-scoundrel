use bevy::prelude::*;

use crate::game::{combat::PlayerState, deck::Deck, room::Room};
use crate::ui::GameState;

/// Marker for the HUD root entity.
#[derive(Component)]
pub struct HudRoot;

/// Marker for HP display.
#[derive(Component)]
pub struct HpDisplay;

/// Marker for weapon display.
#[derive(Component)]
pub struct WeaponDisplay;

/// Marker for deck count display.
#[derive(Component)]
pub struct DeckDisplay;

/// Marker for tiles played counter.
#[derive(Component)]
pub struct TilesPlayedDisplay;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup_hud);
    app.add_systems(
        Update,
        (
            update_hp_display,
            update_weapon_display,
            update_deck_display,
            update_tiles_played_display,
        )
            .run_if(in_state(GameState::Playing)),
    );
    app.add_systems(OnExit(GameState::Playing), cleanup_hud);
}

fn setup_hud(mut commands: Commands) {
    // Create HUD container at top of screen
    commands
        .spawn((
            Name::new("HUD"),
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                padding: UiRect::horizontal(Val::Px(40.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Left side: HP and Weapon
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..default()
                },))
                .with_children(|left| {
                    // HP Display
                    left.spawn((
                        HpDisplay,
                        Text::new("HP: 20/20"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.2, 0.8, 0.8)),
                    ));

                    // Weapon Display
                    left.spawn((
                        WeaponDisplay,
                        Text::new("Weapon: None"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.2, 0.7, 0.3)),
                    ));
                });

            // Center: Game title
            parent.spawn((
                Text::new("MAHJONG SCOUNDREL"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgba(0.9, 0.9, 0.9, 0.5)),
            ));

            // Right side: Deck and tiles played
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    align_items: AlignItems::End,
                    ..default()
                },))
                .with_children(|right| {
                    // Deck count
                    right.spawn((
                        DeckDisplay,
                        Text::new("Deck: 30"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.9, 0.9, 0.9, 0.7)),
                    ));

                    // Tiles played this room
                    right.spawn((
                        TilesPlayedDisplay,
                        Text::new("Played: 0/3"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.9, 0.9, 0.9, 0.7)),
                    ));
                });
        });

    // Bottom HUD with instructions
    commands.spawn((
        HudRoot,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Click tiles to play • Must play 3+ adjacent tiles to clear room • ESC to pause"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgba(0.6, 0.6, 0.6, 0.7)),
        ));
    });
}

fn update_hp_display(player: Res<PlayerState>, mut query: Query<&mut Text, With<HpDisplay>>) {
    if !player.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        **text = format!("HP: {}/{}", player.hp, player.max_hp);
    }
}

fn update_weapon_display(
    player: Res<PlayerState>,
    mut query: Query<(&mut Text, &mut TextColor), With<WeaponDisplay>>,
) {
    if !player.is_changed() {
        return;
    }

    for (mut text, mut color) in query.iter_mut() {
        match &player.weapon {
            Some(weapon) => {
                let value = player.effective_weapon_value();
                let boosted = if player.weapon_boosted { " (2x)" } else { "" };
                **text = format!("Weapon: {} {}{}", weapon.glyph(), value, boosted);
                *color = TextColor(weapon.color());
            }
            None => {
                **text = "Weapon: None".to_string();
                *color = TextColor(Color::srgba(0.5, 0.5, 0.5, 0.7));
            }
        }
    }
}

fn update_deck_display(deck: Res<Deck>, mut query: Query<&mut Text, With<DeckDisplay>>) {
    if !deck.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        **text = format!("Deck: {}", deck.remaining());
    }
}

fn update_tiles_played_display(
    room: Res<Room>,
    mut query: Query<(&mut Text, &mut TextColor), With<TilesPlayedDisplay>>,
) {
    if !room.is_changed() {
        return;
    }

    for (mut text, mut color) in query.iter_mut() {
        let played = room.tiles_played;
        let required = 3;

        **text = format!("Played: {}/{}", played, required);

        // Color green when requirement met
        if played >= required {
            *color = TextColor(Color::srgb(0.2, 0.8, 0.2));
        } else {
            *color = TextColor(Color::srgba(0.9, 0.9, 0.9, 0.7));
        }
    }
}

fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
