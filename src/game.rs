use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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

#[derive(Component)]
struct RedPlayer;

#[derive(Component)]
struct BluePlayer;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (setup_basedata, setup_player).chain(),
        );
    }
}

fn setup_basedata(mut commands: Commands) {
    for index_y in 0..BRICK_COUNT_WIDTH {
        let real_y = (index_y - MID_POS) * BRICK_WIDTH;
        for index_x in 0..BRICK_COUNT_WIDTH {
            let real_x = (index_x - MID_POS) * BRICK_WIDTH;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: if real_x > 0 {
                            Color::GRAY
                        } else {
                            Color::TOMATO
                        },
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

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const LEN: i32 = BRICK_COUNT_WIDTH / 4;
    const RED_X: i32 = -1 * MID_POS * BRICK_WIDTH + LEN * BRICK_WIDTH;
    const BLUE_X: i32 = -1 * RED_X;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform {
                translation: Vec3 {
                    x: RED_X as f32,
                    y: 0.,
                    z: 1.,
                },
                scale : Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 2.,
                },
                ..default()
            },
            ..default()
        },
        RedPlayer,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PINK)),
            transform: Transform {
                translation: Vec3 {
                    x: BLUE_X as f32,
                    y: 0.,
                    z: 1.,
                },
                scale : Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 2.,
                },
                ..default()
            },
            ..default()
        },
        RedPlayer,
    ));
}
