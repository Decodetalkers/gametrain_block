use bevy::prelude::*;

use crate::{
    common::{FIRASANS_FONT, NORMAL_BUTTON, TEXT_COLOR},
    utils::{common_button_system, despawn_with_component},
    GameState,
};

pub struct RpsGamePlugin;

impl Plugin for RpsGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RpsGame), setup_basedata)
            .add_systems(
                OnExit(GameState::RpsGame),
                despawn_with_component::<ReturnButton>,
            )
            .add_systems(
                Update,
                (common_button_system, menu_action)
                    .chain()
                    .run_if(in_state(GameState::RpsGame)),
            );
    }
}

#[derive(Component)]
struct ReturnButton;

fn setup_basedata(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.),
                ..default()
            },
            TextColor(NORMAL_BUTTON),
            ReturnButton,
        ))
        .with_children(|parent| {
            let font = asset_server.load(FIRASANS_FONT);
            let button_icon_style = Node {
                width: Val::Px(30.0),
                height: Val::Auto,
                position_type: PositionType::Relative,
                ..default()
            };
            let button_text_style = TextFont {
                font: font.clone(),
                font_size: 40.0,
                ..Default::default()
            };

            let image = asset_server.load("right.png");
            parent.spawn((
                ImageNode {
                    image,
                    ..Default::default()
                },
                button_icon_style,
                button_text_style.clone(),
            ));
            parent.spawn((
                Text::new("GoBack"),
                button_text_style,
                TextColor(TEXT_COLOR),
            ));
        });
}

fn menu_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Menu);
        }
    }
}
