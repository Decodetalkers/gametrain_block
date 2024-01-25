use bevy::prelude::*;

use crate::common::*;
use crate::utils;
use crate::GameState;

use utils::EntitySpawner;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), main_menu_setup)
            .add_systems(Update, utils::common_button_system);
    }
}
#[derive(Component)]
enum MenuButtonAction {
    EasyPlay,
    NormalPlay,
    HardPlay,
    Help,
    Quit,
}

#[derive(Component)]
struct OnMainMenuScreen;

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::px(120., 120., 10., 30.),
                        ..default()
                    },
                    background_color: BACKGROUND.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "TETRIS",
                            TextStyle {
                                font: font.clone(),
                                font_size: 80.0,
                                color: TEXT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    // Display three buttons for each action available from the main menu:
                    // - Easy Mode
                    // - Normal Mode
                    // - Hard Mode
                    // - Help
                    // - quit
                    parent.spawn_button(
                        MenuButtonAction::EasyPlay,
                        "right.png",
                        "Easy",
                        &asset_server,
                    );
                    parent.spawn_button(
                        MenuButtonAction::NormalPlay,
                        "right.png",
                        "Normal",
                        &asset_server,
                    );
                    parent.spawn_button(
                        MenuButtonAction::HardPlay,
                        "right.png",
                        "Hard",
                        &asset_server,
                    );
                    parent.spawn_button(
                        MenuButtonAction::Help,
                        "wrench.png",
                        "How To Play",
                        &asset_server,
                    );
                    parent.spawn_button(
                        MenuButtonAction::Quit,
                        "exitRight.png",
                        "Quit",
                        &asset_server,
                    );
                });
        });
}
