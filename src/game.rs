use bevy::prelude::*;

use crate::GameState;

#[allow(unused)]
enum Color {
    Red,
    Blue,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), set_up_game_data);
    }
}

#[allow(unused)]
fn set_up_game_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
}
