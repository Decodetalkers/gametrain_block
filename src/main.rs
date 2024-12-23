mod common;
mod menu;
mod region_game;
mod rps_game;

mod utils;
use bevy::{prelude::*, window::WindowResolution};

const WINDOW_WIDTH: f32 = 1100.;
const WINDOW_HEIGHT: f32 = 1000.;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    RegionGame,
    RpsGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "games collections".to_string(),
                resizable: false,
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, camera_setup)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(region_game::RegionGamePlugin)
        .add_plugins(rps_game::RpsGamePlugin)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
