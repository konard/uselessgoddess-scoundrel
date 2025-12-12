use bevy::prelude::*;

use super::{
    combat::{
        CombatEvent, EquipWeaponEvent, GameOverEvent, HealEvent, PlayerState, calculate_combat,
    },
    deck::Deck,
    tiles::TileType,
};
use crate::core::ShakeEvent;
use crate::ui::GameState;

/// Number of tiles dealt per room.
pub const ROOM_SIZE: usize = 4;

/// Minimum tiles that must be played to clear a room.
pub const MIN_TILES_TO_CLEAR: usize = 3;

/// Resource representing the current room state.
#[derive(Resource, Default)]
pub struct Room {
    pub tiles: Vec<Option<TileType>>,
    pub tiles_played: usize,
    pub last_played_index: Option<usize>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            tiles: vec![None; ROOM_SIZE],
            tiles_played: 0,
            last_played_index: None,
        }
    }

    /// Deal new tiles to fill the room from the deck.
    pub fn deal(&mut self, deck: &mut Deck) {
        for slot in self.tiles.iter_mut() {
            if slot.is_none() {
                *slot = deck.draw();
            }
        }
        self.tiles_played = 0;
        self.last_played_index = None;
    }

    /// Check if a tile slot is adjacent to the last played tile (or any slot if no tile played yet).
    pub fn is_valid_selection(&self, index: usize) -> bool {
        if index >= ROOM_SIZE {
            return false;
        }

        // Must have a tile in the slot
        if self.tiles[index].is_none() {
            return false;
        }

        // First tile can be any slot
        if self.last_played_index.is_none() {
            return true;
        }

        // Subsequent tiles must be adjacent to the last played
        let last = self.last_played_index.unwrap();
        index == last.saturating_sub(1) || index == last + 1
    }

    /// Play a tile at the given index.
    pub fn play_tile(&mut self, index: usize) -> Option<TileType> {
        if !self.is_valid_selection(index) {
            return None;
        }

        let tile = self.tiles[index].take();
        if tile.is_some() {
            self.tiles_played += 1;
            self.last_played_index = Some(index);
        }
        tile
    }

    /// Check if the minimum tiles have been played to allow room clear.
    pub fn can_clear(&self) -> bool {
        self.tiles_played >= MIN_TILES_TO_CLEAR
    }

    /// Check if all tiles have been played.
    pub fn is_empty(&self) -> bool {
        self.tiles.iter().all(|t| t.is_none())
    }

    /// Get count of remaining tiles in room.
    pub fn remaining_tiles(&self) -> usize {
        self.tiles.iter().filter(|t| t.is_some()).count()
    }
}

/// Event to trigger dealing a new room.
#[derive(Event)]
pub struct DealRoomEvent;

/// Event to trigger playing a tile.
#[derive(Event)]
pub struct PlayTileEvent {
    pub slot_index: usize,
}

/// Event for escaping the room (Green Dragon).
#[derive(Event)]
pub struct EscapeRoomEvent;

pub fn plugin(app: &mut App) {
    app.insert_resource(Room::new());
    app.add_event::<DealRoomEvent>();
    app.add_event::<PlayTileEvent>();
    app.add_event::<EscapeRoomEvent>();
    app.add_systems(
        Update,
        (
            handle_deal_room,
            handle_play_tile,
            handle_escape_room,
            check_game_over,
        )
            .run_if(in_state(GameState::Playing)),
    );
}

fn handle_deal_room(
    mut events: EventReader<DealRoomEvent>,
    mut room: ResMut<Room>,
    mut deck: ResMut<Deck>,
) {
    for _ in events.read() {
        room.deal(&mut deck);
    }
}

fn handle_play_tile(
    mut events: EventReader<PlayTileEvent>,
    mut room: ResMut<Room>,
    mut player: ResMut<PlayerState>,
    mut combat_events: EventWriter<CombatEvent>,
    mut heal_events: EventWriter<HealEvent>,
    mut equip_events: EventWriter<EquipWeaponEvent>,
    mut shake_events: EventWriter<ShakeEvent>,
    mut deal_events: EventWriter<DealRoomEvent>,
    mut escape_events: EventWriter<EscapeRoomEvent>,
    deck: Res<Deck>,
) {
    for event in events.read() {
        if let Some(tile) = room.play_tile(event.slot_index) {
            match tile {
                TileType::Dots(value) => {
                    // Fight enemy
                    let result = calculate_combat(&player, value);

                    if result.damage_taken > 0 {
                        player.take_damage(result.damage_taken);
                        shake_events.write(ShakeEvent {
                            intensity: 0.3 + (result.damage_taken as f32 * 0.05),
                        });
                    }

                    if result.weapon_used {
                        player.discard_weapon();
                    }

                    combat_events.write(CombatEvent {
                        result,
                        enemy_type: tile,
                    });
                }
                TileType::Bamboo(_) => {
                    // Equip weapon
                    player.equip_weapon(tile);
                    equip_events.write(EquipWeaponEvent { weapon: tile });
                }
                TileType::Characters(value) => {
                    // Heal
                    let heal_amount = value as i32;
                    player.heal(heal_amount);
                    heal_events.write(HealEvent {
                        amount: heal_amount,
                        is_full_heal: false,
                    });
                }
                TileType::RedDragon => {
                    // Full heal
                    player.full_heal();
                    heal_events.write(HealEvent {
                        amount: player.max_hp,
                        is_full_heal: true,
                    });
                }
                TileType::GreenDragon => {
                    // Escape room
                    escape_events.write(EscapeRoomEvent);
                }
                TileType::WhiteDragon => {
                    // Boost weapon
                    player.boost_weapon();
                }
            }

            // Check if we should deal a new room
            if room.can_clear() && room.is_empty() && !deck.is_empty() {
                deal_events.write(DealRoomEvent);
            }
        }
    }
}

fn handle_escape_room(
    mut events: EventReader<EscapeRoomEvent>,
    mut room: ResMut<Room>,
    deck: Res<Deck>,
    mut deal_events: EventWriter<DealRoomEvent>,
) {
    for _ in events.read() {
        // Clear all tiles in current room
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

fn check_game_over(
    player: Res<PlayerState>,
    deck: Res<Deck>,
    room: Res<Room>,
    mut game_over_events: EventWriter<GameOverEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player.is_dead() {
        game_over_events.write(GameOverEvent { victory: false });
        next_state.set(GameState::GameOver);
    } else if deck.is_empty() && room.is_empty() {
        game_over_events.write(GameOverEvent { victory: true });
        next_state.set(GameState::GameOver);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_deal() {
        let mut room = Room::new();
        let mut deck = Deck::default();

        room.deal(&mut deck);

        // All slots should be filled
        assert_eq!(room.remaining_tiles(), ROOM_SIZE);
        assert_eq!(deck.remaining(), 30 - ROOM_SIZE);
    }

    #[test]
    fn test_valid_selection_first_tile() {
        let mut room = Room::new();
        let mut deck = Deck::default();
        room.deal(&mut deck);

        // Any slot is valid for first tile
        assert!(room.is_valid_selection(0));
        assert!(room.is_valid_selection(1));
        assert!(room.is_valid_selection(2));
        assert!(room.is_valid_selection(3));
    }

    #[test]
    fn test_valid_selection_adjacent() {
        let mut room = Room::new();
        let mut deck = Deck::default();
        room.deal(&mut deck);

        // Play tile at index 1
        room.play_tile(1);

        // Only indices 0 and 2 should be valid now
        assert!(room.is_valid_selection(0));
        assert!(!room.is_valid_selection(1)); // Already played
        assert!(room.is_valid_selection(2));
        assert!(!room.is_valid_selection(3)); // Not adjacent
    }

    #[test]
    fn test_room_clear_requirement() {
        let mut room = Room::new();
        let mut deck = Deck::default();
        room.deal(&mut deck);

        assert!(!room.can_clear());

        // Play minimum required tiles
        room.play_tile(0);
        assert!(!room.can_clear());

        room.play_tile(1);
        assert!(!room.can_clear());

        room.play_tile(2);
        assert!(room.can_clear());
    }
}
