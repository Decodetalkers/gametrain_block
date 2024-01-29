use bevy::prelude::*;

use crate::common::*;
use crate::utils;
use crate::utils::{common_button_system, despawn_with_component};
use crate::GameState;

use utils::EntitySpawner;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Menu),
            despawn_with_component::<OnMainMenuScreen>,
        )
        .add_systems(OnEnter(GameState::Menu), main_menu_setup)
        .add_systems(
            Update,
            (common_button_system, menu_action).run_if(in_state(GameState::Menu)),
        );
    }
}
#[derive(Component)]
enum MenuButtonAction {
    RegionBattle,
    RPSBattle,
    HardPlay,
    Help,
    Quit,
}

#[derive(Component)]
struct OnMainMenuScreen;

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FIRASANS_FONT);

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
                            "GAMES COLLICATION",
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
                        MenuButtonAction::RegionBattle,
                        "right.png",
                        "RegionBattle",
                        &asset_server,
                    );
                    parent.spawn_button(
                        MenuButtonAction::RPSBattle,
                        "right.png",
                        "Rock, paper, scissors!",
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

#[allow(clippy::type_complexity)]
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::RegionBattle => game_state.set(GameState::RegionGame),
                MenuButtonAction::RPSBattle => game_state.set(GameState::RpsGame),
                _ => {}
            }
        }
    }
}
