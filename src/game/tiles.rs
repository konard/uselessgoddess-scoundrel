use bevy::prelude::*;

/// Represents the different types of Mahjong tiles used in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    /// Dots (Enemies) - Values 1-9, deal damage to player
    Dots(u8),
    /// Bamboo (Weapons) - Values 1-9, equip to fight enemies
    Bamboo(u8),
    /// Characters (Health/Potions) - Values 1-9, restore HP
    Characters(u8),
    /// Red Dragon - Full Heal
    RedDragon,
    /// Green Dragon - Escape Room (skip current tiles)
    GreenDragon,
    /// White Dragon - Double Weapon Value
    WhiteDragon,
}

impl TileType {
    /// Get the numeric value of the tile (1-9 for suited tiles, 0 for dragons).
    pub fn value(&self) -> u8 {
        match self {
            TileType::Dots(v) | TileType::Bamboo(v) | TileType::Characters(v) => *v,
            TileType::RedDragon | TileType::GreenDragon | TileType::WhiteDragon => 0,
        }
    }

    /// Check if this tile is an enemy (Dots).
    pub fn is_enemy(&self) -> bool {
        matches!(self, TileType::Dots(_))
    }

    /// Check if this tile is a weapon (Bamboo).
    pub fn is_weapon(&self) -> bool {
        matches!(self, TileType::Bamboo(_))
    }

    /// Check if this tile is a health potion (Characters).
    pub fn is_health(&self) -> bool {
        matches!(self, TileType::Characters(_))
    }

    /// Check if this tile is a dragon (special effect).
    pub fn is_dragon(&self) -> bool {
        matches!(
            self,
            TileType::RedDragon | TileType::GreenDragon | TileType::WhiteDragon
        )
    }

    /// Get the Unicode glyph for this tile.
    pub fn glyph(&self) -> &'static str {
        match self {
            // Dots tiles - using Mahjong Unicode characters
            TileType::Dots(1) => "🀙",
            TileType::Dots(2) => "🀚",
            TileType::Dots(3) => "🀛",
            TileType::Dots(4) => "🀜",
            TileType::Dots(5) => "🀝",
            TileType::Dots(6) => "🀞",
            TileType::Dots(7) => "🀟",
            TileType::Dots(8) => "🀠",
            TileType::Dots(9) => "🀡",
            // Bamboo tiles
            TileType::Bamboo(1) => "🀐",
            TileType::Bamboo(2) => "🀑",
            TileType::Bamboo(3) => "🀒",
            TileType::Bamboo(4) => "🀓",
            TileType::Bamboo(5) => "🀔",
            TileType::Bamboo(6) => "🀕",
            TileType::Bamboo(7) => "🀖",
            TileType::Bamboo(8) => "🀗",
            TileType::Bamboo(9) => "🀘",
            // Characters tiles
            TileType::Characters(1) => "🀇",
            TileType::Characters(2) => "🀈",
            TileType::Characters(3) => "🀉",
            TileType::Characters(4) => "🀊",
            TileType::Characters(5) => "🀋",
            TileType::Characters(6) => "🀌",
            TileType::Characters(7) => "🀍",
            TileType::Characters(8) => "🀎",
            TileType::Characters(9) => "🀏",
            // Dragons
            TileType::RedDragon => "🀄",
            TileType::GreenDragon => "🀅",
            TileType::WhiteDragon => "🀆",
            // Fallback for invalid values
            _ => "?",
        }
    }

    /// Get the display name for this tile type.
    pub fn name(&self) -> String {
        match self {
            TileType::Dots(v) => format!("{} Dots", v),
            TileType::Bamboo(v) => format!("{} Bamboo", v),
            TileType::Characters(v) => format!("{} Characters", v),
            TileType::RedDragon => "Red Dragon".to_string(),
            TileType::GreenDragon => "Green Dragon".to_string(),
            TileType::WhiteDragon => "White Dragon".to_string(),
        }
    }

    /// Get the color for rendering this tile.
    pub fn color(&self) -> Color {
        match self {
            // Enemies are red (dim)
            TileType::Dots(_) => Color::srgb(0.8, 0.2, 0.2),
            // Weapons are green (dim)
            TileType::Bamboo(_) => Color::srgb(0.2, 0.7, 0.3),
            // Health is cyan
            TileType::Characters(_) => Color::srgb(0.2, 0.8, 0.8),
            // Dragons have special colors
            TileType::RedDragon => Color::srgb(1.0, 0.3, 0.3),
            TileType::GreenDragon => Color::srgb(0.3, 1.0, 0.3),
            TileType::WhiteDragon => Color::srgb(0.9, 0.9, 0.9),
        }
    }
}

/// Component representing a tile entity in the game.
#[derive(Component, Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub slot_index: usize,
}

pub fn plugin(app: &mut App) {
    // Tile module doesn't need systems, just component definitions
    let _ = app;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_values() {
        assert_eq!(TileType::Dots(5).value(), 5);
        assert_eq!(TileType::Bamboo(9).value(), 9);
        assert_eq!(TileType::Characters(1).value(), 1);
        assert_eq!(TileType::RedDragon.value(), 0);
    }

    #[test]
    fn test_tile_types() {
        assert!(TileType::Dots(3).is_enemy());
        assert!(!TileType::Dots(3).is_weapon());

        assert!(TileType::Bamboo(5).is_weapon());
        assert!(!TileType::Bamboo(5).is_enemy());

        assert!(TileType::Characters(7).is_health());
        assert!(!TileType::Characters(7).is_dragon());

        assert!(TileType::RedDragon.is_dragon());
        assert!(TileType::GreenDragon.is_dragon());
        assert!(TileType::WhiteDragon.is_dragon());
    }

    #[test]
    fn test_tile_glyphs() {
        assert_eq!(TileType::Dots(1).glyph(), "🀙");
        assert_eq!(TileType::Bamboo(1).glyph(), "🀐");
        assert_eq!(TileType::Characters(1).glyph(), "🀇");
        assert_eq!(TileType::RedDragon.glyph(), "🀄");
    }
}
