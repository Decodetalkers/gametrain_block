mod common;
mod menu;

mod utils;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "hello".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, camera_setup)
        .add_state::<GameState>()
        .add_plugins(menu::MenuPlugin)
        .run()
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
