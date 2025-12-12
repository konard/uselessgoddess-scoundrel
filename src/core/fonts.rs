use bevy::prelude::*;

/// Resource holding handles to the game fonts.
#[derive(Resource)]
pub struct GameFonts {
    /// Font with Mahjong Unicode symbols (Noto Sans Symbols 2).
    pub mahjong: Handle<Font>,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_fonts);
}

fn setup_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mahjong_font = asset_server.load("fonts/NotoSansSymbols2-Regular.ttf");

    commands.insert_resource(GameFonts {
        mahjong: mahjong_font,
    });
}
