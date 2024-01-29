mod common;
mod game;
mod menu;

mod utils;
use bevy::{prelude::*, window::WindowResolution};

const WINDOW_WIDTH: f32 = 1100.;
const WINDOW_HEIGHT: f32 = 1000.;

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
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, camera_setup)
        .add_state::<GameState>()
        .add_plugins(menu::MenuPlugin)
        .add_plugins(game::GamePlugin)
        .run()
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
