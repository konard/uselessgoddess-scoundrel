use bevy::prelude::*;

use crate::core::GameFonts;
use crate::game::{
    combat::PlayerState,
    deck::Deck,
    room::{DealRoomEvent, Room},
};
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

/// Marker for the "Clear Room" button.
#[derive(Component)]
pub struct ClearRoomButton;

/// Marker for the "Clear Room" button container (for visibility toggling).
#[derive(Component)]
pub struct ClearRoomButtonContainer;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup_hud);
    app.add_systems(
        Update,
        (
            update_hp_display,
            update_weapon_display,
            update_deck_display,
            update_tiles_played_display,
            update_clear_room_button_visibility,
            handle_clear_room_button,
        )
            .run_if(in_state(GameState::Playing)),
    );
    // Only cleanup HUD when going to MainMenu, not when pausing
    app.add_systems(OnEnter(GameState::MainMenu), cleanup_hud);
    app.add_systems(OnEnter(GameState::GameOver), cleanup_hud);
}

fn setup_hud(mut commands: Commands, fonts: Option<Res<GameFonts>>) {
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

                    // Weapon Display - use Mahjong font for glyph display
                    let mut weapon_font = TextFont {
                        font_size: 18.0,
                        ..default()
                    };
                    if let Some(ref f) = fonts {
                        weapon_font.font = f.mahjong.clone();
                    }
                    left.spawn((
                        WeaponDisplay,
                        Text::new("Weapon: None"),
                        weapon_font,
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

    // Bottom HUD with instructions and Clear Room button
    commands
        .spawn((
            HudRoot,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(15.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Clear Room button (hidden by default, shown when 3+ tiles played)
            parent
                .spawn((
                    ClearRoomButtonContainer,
                    Node {
                        ..default()
                    },
                    Visibility::Hidden,
                ))
                .with_children(|container| {
                    container
                        .spawn((
                            ClearRoomButton,
                            Button,
                            Node {
                                padding: UiRect::axes(Val::Px(30.0), Val::Px(12.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.95)),
                            BorderColor::all(Color::srgb(0.2, 0.8, 0.2)),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("[ CLEAR ROOM ]"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.2, 0.8, 0.2)),
                            ));
                        });
                });

            // Instructions
            parent.spawn((
                Text::new(
                    "Click tiles to play • Must play 3+ adjacent tiles to clear room • ESC to pause",
                ),
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

/// Show the "Clear Room" button when 3+ tiles have been played and there are remaining tiles.
fn update_clear_room_button_visibility(
    room: Res<Room>,
    deck: Res<Deck>,
    mut query: Query<&mut Visibility, With<ClearRoomButtonContainer>>,
) {
    // Show button when:
    // 1. Player has played 3+ tiles (can clear)
    // 2. There are remaining tiles in the room
    // 3. There are tiles in the deck OR remaining tiles
    let should_show = room.can_clear() && room.remaining_tiles() > 0 && !deck.is_empty();

    for mut visibility in query.iter_mut() {
        *visibility = if should_show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Handle the "Clear Room" button click.
fn handle_clear_room_button(
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<ClearRoomButton>),
    >,
    mut text_query: Query<&mut TextColor>,
    mut room: ResMut<Room>,
    deck: Res<Deck>,
    mut deal_events: MessageWriter<DealRoomEvent>,
) {
    for (interaction, children) in interaction_query.iter_mut() {
        // Update button text color on hover
        for child in children.iter() {
            if let Ok(mut color) = text_query.get_mut(child) {
                match *interaction {
                    Interaction::Pressed => {
                        *color = TextColor(Color::srgb(0.4, 1.0, 0.4));
                    }
                    Interaction::Hovered => {
                        *color = TextColor(Color::srgb(0.3, 0.9, 0.3));
                    }
                    Interaction::None => {
                        *color = TextColor(Color::srgb(0.2, 0.8, 0.2));
                    }
                }
            }
        }

        // Handle button press
        if *interaction == Interaction::Pressed && room.can_clear() {
            // Clear remaining tiles in the room
            for tile in room.tiles.iter_mut() {
                *tile = None;
            }
            room.tiles_played = 0;
            room.last_played_index = None;

            // Deal new room if deck isn't empty
            if !deck.is_empty() {
                deal_events.write(DealRoomEvent);
            }
        }
    }
}

fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
