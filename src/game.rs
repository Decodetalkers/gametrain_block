use bevy::prelude::*;

use crate::GameState;

const BRICK_WIDTH: i32 = 20;
const BRICK_COUNT_WIDTH: i32 = 30;

const MID_POS: i32 = BRICK_COUNT_WIDTH / 2;

#[allow(unused)]
enum BrickColor {
    Red,
    Blue,
}

pub struct GamePlugin;

#[derive(Component)]
struct Brick(BrickColor);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), set_up_game_data);
    }
}

#[allow(unused)]
fn set_up_game_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    for index_y in 0..BRICK_COUNT_WIDTH {
        let real_y = (index_y - MID_POS) * BRICK_WIDTH;
        for index_x in 0..BRICK_COUNT_WIDTH {
            let real_x = (index_x - MID_POS) * BRICK_WIDTH;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: if real_x > 0 { Color::RED } else { Color::BLUE },
                        ..default()
                    },
                    transform: Transform {
                        scale: Vec3 {
                            x: BRICK_WIDTH as f32,
                            y: BRICK_WIDTH as f32,
                            z: 0.,
                        },
                        translation: Vec3 {
                            x: real_x as f32,
                            y: real_y as f32,
                            z: 0.,
                        },
                        ..default()
                    },
                    ..default()
                },
                Brick(if real_x > 0 {
                    BrickColor::Red
                } else {
                    BrickColor::Blue
                }),
            ));
        }
    }
}
