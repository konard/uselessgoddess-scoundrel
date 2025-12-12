use bevy::prelude::*;

use super::tiles::TileType;

/// Maximum HP the player can have.
pub const MAX_HP: i32 = 20;

/// Resource representing the player's state.
#[derive(Resource)]
pub struct PlayerState {
    pub hp: i32,
    pub max_hp: i32,
    pub weapon: Option<TileType>,
    pub weapon_boosted: bool,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            hp: MAX_HP,
            max_hp: MAX_HP,
            weapon: None,
            weapon_boosted: false,
        }
    }

    /// Get the effective weapon value (doubled if boosted by White Dragon).
    pub fn effective_weapon_value(&self) -> u8 {
        if let Some(weapon) = &self.weapon {
            let base_value = weapon.value();
            if self.weapon_boosted {
                base_value.saturating_mul(2)
            } else {
                base_value
            }
        } else {
            0
        }
    }

    /// Heal the player by the given amount, capped at max HP.
    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    /// Fully heal the player.
    pub fn full_heal(&mut self) {
        self.hp = self.max_hp;
    }

    /// Apply damage to the player.
    pub fn take_damage(&mut self, amount: i32) {
        self.hp = (self.hp - amount).max(0);
    }

    /// Check if the player is dead.
    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    /// Equip a weapon, replacing any existing weapon.
    pub fn equip_weapon(&mut self, weapon: TileType) {
        self.weapon = Some(weapon);
        self.weapon_boosted = false;
    }

    /// Discard the current weapon.
    pub fn discard_weapon(&mut self) {
        self.weapon = None;
        self.weapon_boosted = false;
    }

    /// Boost the current weapon (White Dragon effect).
    pub fn boost_weapon(&mut self) {
        if self.weapon.is_some() {
            self.weapon_boosted = true;
        }
    }
}

/// Result of combat between player and enemy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatResult {
    pub damage_taken: i32,
    pub weapon_used: bool,
    pub enemy_killed: bool,
}

/// Calculate the result of fighting an enemy.
///
/// Rules:
/// - If no weapon: take full enemy damage
/// - If weapon equipped:
///   - Weapon value reduces enemy damage
///   - If weapon >= enemy: take 0 damage
///   - Weapon is ALWAYS discarded after use
pub fn calculate_combat(player: &PlayerState, enemy_value: u8) -> CombatResult {
    let weapon_value = player.effective_weapon_value();

    if weapon_value == 0 {
        // No weapon - take full damage
        CombatResult {
            damage_taken: enemy_value as i32,
            weapon_used: false,
            enemy_killed: true,
        }
    } else {
        // Weapon equipped
        let damage = (enemy_value as i32 - weapon_value as i32).max(0);
        CombatResult {
            damage_taken: damage,
            weapon_used: true,
            enemy_killed: true,
        }
    }
}

/// Event triggered when combat occurs.
#[derive(Event)]
pub struct CombatEvent {
    pub result: CombatResult,
    pub enemy_type: TileType,
}

/// Event triggered when player heals.
#[derive(Event)]
pub struct HealEvent {
    pub amount: i32,
    pub is_full_heal: bool,
}

/// Event triggered when player equips a weapon.
#[derive(Event)]
pub struct EquipWeaponEvent {
    pub weapon: TileType,
}

/// Event triggered when the game ends.
#[derive(Event)]
pub struct GameOverEvent {
    pub victory: bool,
}

pub fn plugin(app: &mut App) {
    app.insert_resource(PlayerState::new());
    app.add_event::<CombatEvent>();
    app.add_event::<HealEvent>();
    app.add_event::<EquipWeaponEvent>();
    app.add_event::<GameOverEvent>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combat_no_weapon() {
        let player = PlayerState::new();
        let result = calculate_combat(&player, 5);

        assert_eq!(result.damage_taken, 5);
        assert!(!result.weapon_used);
        assert!(result.enemy_killed);
    }

    #[test]
    fn test_combat_with_weapon_stronger() {
        let mut player = PlayerState::new();
        player.equip_weapon(TileType::Bamboo(7));

        let result = calculate_combat(&player, 5);

        assert_eq!(result.damage_taken, 0);
        assert!(result.weapon_used);
        assert!(result.enemy_killed);
    }

    #[test]
    fn test_combat_with_weapon_weaker() {
        let mut player = PlayerState::new();
        player.equip_weapon(TileType::Bamboo(3));

        let result = calculate_combat(&player, 7);

        assert_eq!(result.damage_taken, 4);
        assert!(result.weapon_used);
        assert!(result.enemy_killed);
    }

    #[test]
    fn test_combat_with_weapon_equal() {
        let mut player = PlayerState::new();
        player.equip_weapon(TileType::Bamboo(5));

        let result = calculate_combat(&player, 5);

        assert_eq!(result.damage_taken, 0);
        assert!(result.weapon_used);
        assert!(result.enemy_killed);
    }

    #[test]
    fn test_boosted_weapon() {
        let mut player = PlayerState::new();
        player.equip_weapon(TileType::Bamboo(4));
        player.boost_weapon();

        assert_eq!(player.effective_weapon_value(), 8);

        let result = calculate_combat(&player, 7);
        assert_eq!(result.damage_taken, 0);
    }

    #[test]
    fn test_healing() {
        let mut player = PlayerState::new();
        player.take_damage(10);
        assert_eq!(player.hp, 10);

        player.heal(5);
        assert_eq!(player.hp, 15);

        // Healing should cap at max HP
        player.heal(100);
        assert_eq!(player.hp, MAX_HP);
    }

    #[test]
    fn test_full_heal() {
        let mut player = PlayerState::new();
        player.take_damage(15);
        assert_eq!(player.hp, 5);

        player.full_heal();
        assert_eq!(player.hp, MAX_HP);
    }

    #[test]
    fn test_player_death() {
        let mut player = PlayerState::new();
        assert!(!player.is_dead());

        player.take_damage(MAX_HP);
        assert!(player.is_dead());
    }

    #[test]
    fn test_weapon_discard() {
        let mut player = PlayerState::new();
        player.equip_weapon(TileType::Bamboo(5));
        assert!(player.weapon.is_some());

        player.discard_weapon();
        assert!(player.weapon.is_none());
        assert_eq!(player.effective_weapon_value(), 0);
    }
}
