use bevy::prelude::*;

use crate::game::{combat::PlayerState, deck::Deck, room::Room};
use crate::ui::GameState;

/// Marker for main menu root entity.
#[derive(Component)]
pub struct MainMenuRoot;

/// Marker for pause menu root entity.
#[derive(Component)]
pub struct PauseMenuRoot;

/// Marker for game over menu root entity.
#[derive(Component)]
pub struct GameOverRoot;

/// Button actions.
#[derive(Component, Clone, Copy)]
pub enum MenuButton {
    Play,
    Resume,
    MainMenu,
    Quit,
}

pub fn plugin(app: &mut App) {
    // Main menu
    app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
    app.add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);

    // Pause menu
    app.add_systems(OnEnter(GameState::Paused), setup_pause_menu);
    app.add_systems(OnExit(GameState::Paused), cleanup_pause_menu);

    // Game over menu
    app.add_systems(OnEnter(GameState::GameOver), setup_game_over);
    app.add_systems(OnExit(GameState::GameOver), cleanup_game_over);

    // Button interaction
    app.add_systems(Update, (handle_button_interactions, handle_pause_input));
}

/// Helper macro for spawning buttons to avoid type issues with ChildBuilder
macro_rules! spawn_menu_button {
    ($parent:expr, $action:expr, $text:expr) => {
        $parent
            .spawn((
                $action,
                Button,
                Node {
                    padding: UiRect::all(Val::Px(15.0)),
                    min_width: Val::Px(200.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new($text),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
            });
    };
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Name::new("MainMenu"),
            MainMenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(30.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.06, 0.06, 0.98)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("MAHJONG"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            parent.spawn((
                Text::new("SCOUNDREL"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.2, 0.2)),
            ));

            // Subtitle
            parent.spawn((
                Text::new("A roguelike tile game"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgba(0.7, 0.7, 0.7, 0.8)),
            ));

            // Spacing
            parent.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            // Play button
            spawn_menu_button!(parent, MenuButton::Play, "[ PLAY ]");

            // Quit button
            spawn_menu_button!(parent, MenuButton::Quit, "[ QUIT ]");

            // Instructions
            parent.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            parent.spawn((
                Text::new("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgba(0.4, 0.4, 0.4, 0.5)),
            ));

            parent.spawn((
                Text::new("Survive the dungeon by playing Mahjong tiles"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgba(0.6, 0.6, 0.6, 0.8)),
            ));

            parent.spawn((
                Text::new("🀙 Dots = Enemies  •  🀐 Bamboo = Weapons  •  🀇 Characters = Healing"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgba(0.5, 0.5, 0.5, 0.7)),
            ));
        });
}

fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            Name::new("PauseMenu"),
            PauseMenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            parent.spawn(Node {
                height: Val::Px(20.0),
                ..default()
            });

            spawn_menu_button!(parent, MenuButton::Resume, "[ RESUME ]");
            spawn_menu_button!(parent, MenuButton::MainMenu, "[ MAIN MENU ]");
        });
}

fn setup_game_over(
    mut commands: Commands,
    player: Res<PlayerState>,
    mut deck: ResMut<Deck>,
    mut room: ResMut<Room>,
) {
    let victory = !player.is_dead();

    commands
        .spawn((
            Name::new("GameOver"),
            GameOverRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
        ))
        .with_children(|parent| {
            if victory {
                parent.spawn((
                    Text::new("VICTORY"),
                    TextFont {
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.8, 0.2)),
                ));

                parent.spawn((
                    Text::new("You cleared the dungeon!"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.9, 0.7)),
                ));
            } else {
                parent.spawn((
                    Text::new("GAME OVER"),
                    TextFont {
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.2, 0.2)),
                ));

                parent.spawn((
                    Text::new("The dungeon claims another soul..."),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.7, 0.7)),
                ));
            }

            parent.spawn(Node {
                height: Val::Px(30.0),
                ..default()
            });

            spawn_menu_button!(parent, MenuButton::Play, "[ PLAY AGAIN ]");
            spawn_menu_button!(parent, MenuButton::MainMenu, "[ MAIN MENU ]");
        });

    // Reset game state for next play
    *deck = Deck::default();
    *room = Room::default();
}

fn handle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &MenuButton, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut TextColor>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player: ResMut<PlayerState>,
    mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, button, children) in interaction_query.iter_mut() {
        // Update text color based on interaction
        for child in children.iter() {
            if let Ok(mut color) = text_query.get_mut(child) {
                match *interaction {
                    Interaction::Pressed => {
                        *color = TextColor(Color::srgb(0.2, 0.8, 0.8));
                    }
                    Interaction::Hovered => {
                        *color = TextColor(Color::srgb(0.9, 0.9, 0.9));
                    }
                    Interaction::None => {
                        *color = TextColor(Color::srgb(0.7, 0.7, 0.7));
                    }
                }
            }
        }

        // Handle button press
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::Play => {
                    *player = PlayerState::new();
                    next_state.set(GameState::Playing);
                }
                MenuButton::Resume => {
                    next_state.set(GameState::Playing);
                }
                MenuButton::MainMenu => {
                    next_state.set(GameState::MainMenu);
                }
                MenuButton::Quit => {
                    app_exit.write(AppExit::Success);
                }
            }
        }
    }
}

fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
