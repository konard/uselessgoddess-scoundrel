#![allow(unused_imports)]

pub use core::{fmt::Debug, hash::Hash, marker::PhantomData, time::Duration};

pub use bevy::{
    color::palettes::tailwind::*,
    ecs::spawn::SpawnWith,
    input::common_conditions::*,
    math::{vec2, vec3},
    prelude::*,
    sprite::Anchor,
    window::PrimaryWindow,
};

pub use rand::prelude::*;

// Note: Avoid glob re-exports of modules with `plugin` functions to prevent ambiguity
pub use crate::{
    core::{
        EasingFunction, MouseScreenPosition, MouseWorldPosition, PrimaryCamera,
        ScreenShake, ShakeEvent, Tween,
    },
    game::{
        combat::{
            calculate_combat, CombatEvent, CombatResult, EquipWeaponEvent,
            GameOverEvent, HealEvent, PlayerState,
        },
        deck::Deck,
        room::{DealRoomEvent, EscapeRoomEvent, PlayTileEvent, Room, ROOM_SIZE, MIN_TILES_TO_CLEAR},
        tiles::{Tile, TileType},
    },
    ui::GameState,
};
