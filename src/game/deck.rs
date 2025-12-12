use bevy::prelude::*;
use rand::prelude::*;

use super::tiles::TileType;

/// Resource representing the deck of tiles.
#[derive(Resource)]
pub struct Deck {
    tiles: Vec<TileType>,
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl Deck {
    /// Create a new shuffled deck with all Mahjong tiles.
    pub fn new() -> Self {
        let mut tiles = Vec::new();

        // Add Dots (Enemies) 1-9
        for value in 1..=9 {
            tiles.push(TileType::Dots(value));
        }

        // Add Bamboo (Weapons) 1-9
        for value in 1..=9 {
            tiles.push(TileType::Bamboo(value));
        }

        // Add Characters (Health) 1-9
        for value in 1..=9 {
            tiles.push(TileType::Characters(value));
        }

        // Add Dragons (3 total)
        tiles.push(TileType::RedDragon);
        tiles.push(TileType::GreenDragon);
        tiles.push(TileType::WhiteDragon);

        let mut deck = Self { tiles };
        deck.shuffle();
        deck
    }

    /// Shuffle the deck.
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.tiles.shuffle(&mut rng);
    }

    /// Draw a tile from the deck.
    pub fn draw(&mut self) -> Option<TileType> {
        self.tiles.pop()
    }

    /// Draw multiple tiles from the deck.
    pub fn draw_multiple(&mut self, count: usize) -> Vec<TileType> {
        let mut drawn = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(tile) = self.draw() {
                drawn.push(tile);
            }
        }
        drawn
    }

    /// Check if the deck is empty.
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    /// Get the number of tiles remaining in the deck.
    pub fn remaining(&self) -> usize {
        self.tiles.len()
    }
}

pub fn plugin(app: &mut App) {
    app.insert_resource(Deck::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        // 9 dots + 9 bamboo + 9 characters + 3 dragons = 30 tiles
        assert_eq!(deck.remaining(), 30);
    }

    #[test]
    fn test_draw() {
        let mut deck = Deck::new();
        let initial = deck.remaining();

        let tile = deck.draw();
        assert!(tile.is_some());
        assert_eq!(deck.remaining(), initial - 1);
    }

    #[test]
    fn test_draw_multiple() {
        let mut deck = Deck::new();
        let tiles = deck.draw_multiple(4);
        assert_eq!(tiles.len(), 4);
        assert_eq!(deck.remaining(), 26);
    }

    #[test]
    fn test_draw_empty() {
        let mut deck = Deck::new();
        // Draw all tiles
        for _ in 0..30 {
            deck.draw();
        }
        assert!(deck.is_empty());
        assert!(deck.draw().is_none());
    }
}
