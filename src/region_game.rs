use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        Anchor, MaterialMesh2dBundle,
    },
};

use crate::{utils::despawn_with_component, GameState};

const BRICK_WIDTH: i32 = 20;
const BRICK_COUNT_WIDTH: i32 = 30;

const MID_POS: i32 = BRICK_COUNT_WIDTH / 2;

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

const WALL_THICKNESS: f32 = 40.0;
// x coordinates
const LEFT_WALL: f32 = (-MID_POS * BRICK_WIDTH) as f32;
const RIGHT_WALL: f32 = -1. * LEFT_WALL;
// y coordinates
const BOTTOM_WALL: f32 = LEFT_WALL;
const TOP_WALL: f32 = RIGHT_WALL;

const GAME_DATA_TEXT_COLOR: Color = Color::rgb(0., 0.22, 0.76);

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum BrickColor {
    Red,
    Blue,
}

impl BrickColor {
    fn change(&mut self) {
        match self {
            Self::Red => *self = Self::Blue,
            Self::Blue => *self = Self::Red,
        }
    }
}

pub struct GamePlugin;

#[derive(Component)]
struct Brick(BrickColor);

#[derive(Component)]
struct PlayBoard;
#[derive(Component)]
struct PlayerScore(BrickColor);

trait Player {
    const BREAK_COLOR: BrickColor;
    const RENDER_COLOR: Color;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn place_board(commands: &mut Commands, asset_server: &Res<AssetServer>);
    fn place_player(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    );
}

fn create_text_bundle(msg: &str, x: f32, y: f32, asset_server: &Res<AssetServer>) -> Text2dBundle {
    Text2dBundle {
        text: Text::from_section(
            msg,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 42.0,
                color: GAME_DATA_TEXT_COLOR,
            },
        )
        .with_alignment(TextAlignment::Center),
        transform: Transform {
            translation: Vec3::new(x, y, 0.),
            ..default()
        },
        text_anchor: Anchor::TopCenter,
        ..default()
    }
}

#[derive(Component)]
struct RedPlayer {
    x: f32,
    y: f32,
}

impl RedPlayer {
    fn new() -> Self {
        Self { x: 100., y: 100. }
    }
}

impl Player for RedPlayer {
    const BREAK_COLOR: BrickColor = BrickColor::Red;
    const RENDER_COLOR: Color = Color::TOMATO;
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
    fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    fn place_board(commands: &mut Commands, asset_server: &Res<AssetServer>) {
        const BOARD_LEFT_POS: f32 = (-MID_POS * BRICK_WIDTH - 80) as f32;
        commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(BOARD_LEFT_POS, 0., 0.),
                    ..default()
                },
                PlayBoard,
            ))
            .with_children(|parent| {
                let up_margin: f32 = 60.;
                let top_y: f32 = 180.;
                let x: f32 = -50.;

                parent.spawn(create_text_bundle("RED SCORE", x, top_y, asset_server));
                parent
                    .spawn(create_text_bundle("0", x, top_y - up_margin, asset_server))
                    .insert(PlayerScore(BrickColor::Red));
            });
    }
    fn place_player(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        const LEN: i32 = BRICK_COUNT_WIDTH / 4;
        const RED_X: i32 = -MID_POS * BRICK_WIDTH + LEN * BRICK_WIDTH;
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
                    scale: Vec3 {
                        x: 1.,
                        y: 1.,
                        z: 2.,
                    },
                    ..default()
                },
                ..default()
            },
            RedPlayer::new(),
            Collider,
        ));
    }
}

#[derive(Component)]
struct BluePlayer {
    x: f32,
    y: f32,
}

impl Player for BluePlayer {
    const BREAK_COLOR: BrickColor = BrickColor::Blue;
    const RENDER_COLOR: Color = Color::GRAY;
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
    fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    fn place_board(commands: &mut Commands, asset_server: &Res<AssetServer>) {
        const BOARD_LEFT_POS: f32 = (MID_POS * BRICK_WIDTH + 200) as f32;
        commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(BOARD_LEFT_POS, 0., 0.),
                    ..default()
                },
                PlayBoard,
            ))
            .with_children(|parent| {
                let up_margin: f32 = 60.;
                let top_y: f32 = 180.;
                let x: f32 = -50.;
                parent.spawn(create_text_bundle("BLUE SCORE", x, top_y, asset_server));
                parent
                    .spawn(create_text_bundle("0", x, top_y - up_margin, asset_server))
                    .insert(PlayerScore(BrickColor::Blue));
            });
    }

    fn place_player(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        const LEN: i32 = BRICK_COUNT_WIDTH / 4;
        const RED_X: i32 = -MID_POS * BRICK_WIDTH + LEN * BRICK_WIDTH;
        const BLUE_X: i32 = -RED_X;
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
                    scale: Vec3 {
                        x: 1.,
                        y: 1.,
                        z: 2.,
                    },
                    ..default()
                },
                ..default()
            },
            BluePlayer::new(),
            Collider,
        ));
    }
}

impl BluePlayer {
    fn new() -> Self {
        Self { x: -100., y: -100. }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (setup_basedata, setup_player).chain(),
        )
        .add_systems(
            OnExit(GameState::Game),
            (
                despawn_with_component::<Collider>,
                despawn_with_component::<PlayBoard>,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                handle_move::<RedPlayer>,
                handle_move::<BluePlayer>,
                check_collider::<RedPlayer>,
                check_collider::<BluePlayer>,
                handle_score_update,
            )
                .chain()
                .run_if(in_state(GameState::Game)),
        );
    }
}

fn setup_basedata(mut commands: Commands, asset_server: Res<AssetServer>) {
    for index_y in 0..BRICK_COUNT_WIDTH + 1 {
        let real_y = (index_y - MID_POS) * BRICK_WIDTH;
        for index_x in 0..BRICK_COUNT_WIDTH + 1 {
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
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
    RedPlayer::place_board(&mut commands, &asset_server);
    BluePlayer::place_board(&mut commands, &asset_server);
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    RedPlayer::place_player(&mut commands, &mut meshes, &mut materials);
    BluePlayer::place_player(&mut commands, &mut meshes, &mut materials);
}

fn check_collider<P>(
    mut blocks: Query<(&Transform, &mut Sprite, &mut Brick)>,
    walls: Query<&Transform, With<Collider>>,
    mut player: Query<(&Transform, &mut P), With<P>>,
) where
    P: Player + Component,
{
    let (red_tran, mut red_state) = player.single_mut();
    for (transform, mut spite, mut block) in &mut blocks {
        if let Some(coll) = collide(
            red_tran.translation,
            red_tran.scale.truncate(),
            transform.translation,
            transform.scale.truncate(),
        ) {
            let x = red_state.x();
            let y = red_state.y();
            if block.0 == P::BREAK_COLOR {
                match coll {
                    Collision::Left | Collision::Right => red_state.set_x(-x),
                    Collision::Top | Collision::Bottom => red_state.set_y(-y),
                    Collision::Inside => { /* do nothing */ }
                }

                spite.color = P::RENDER_COLOR;
                block.0.change();
            }
        }
    }
    for transform in &walls {
        if let Some(coll) = collide(
            red_tran.translation,
            red_tran.scale.truncate(),
            transform.translation,
            transform.scale.truncate(),
        ) {
            let x = red_state.x();
            let y = red_state.y();
            match coll {
                Collision::Left | Collision::Right => red_state.set_x(-x),
                Collision::Top | Collision::Bottom => red_state.set_y(-y),
                Collision::Inside => { /* do nothing */ }
            }
        }
    }
}

fn handle_move<P>(mut blue_query: Query<(&mut Transform, &P), With<P>>, timer: Res<Time<Fixed>>)
where
    P: Player + Component,
{
    let (mut player_trans, state) = blue_query.single_mut();

    player_trans.translation.x += state.x() * timer.delta().as_secs_f32();
    player_trans.translation.y += state.y() * timer.delta().as_secs_f32();
}

fn handle_score_update(
    mut text_query: Query<(&mut Text, &PlayerScore), With<PlayerScore>>,
    blocks: Query<&Brick>,
) {
    for (mut text, playerboard) in &mut text_query {
        let count = blocks.iter().filter(|b| b.0 == playerboard.0).count();
        text.sections[0].value = count.to_string();
    }
}
