use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{
    common::{FIRASANS_FONT, NORMAL_BUTTON, TEXT_COLOR},
    utils::{common_button_system, despawn_with_component},
    GameState,
};

const BRICK_WIDTH: i32 = 20;
const BRICK_COUNT_WIDTH: i32 = 30;

const MID_POS: i32 = BRICK_COUNT_WIDTH / 2;

const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

const WALL_THICKNESS: f32 = 40.0;
// x coordinates
const LEFT_WALL: f32 = (-MID_POS * BRICK_WIDTH) as f32;
const RIGHT_WALL: f32 = -1. * LEFT_WALL;
// y coordinates
const BOTTOM_WALL: f32 = LEFT_WALL;
const TOP_WALL: f32 = RIGHT_WALL;

const GAME_DATA_TEXT_COLOR: Color = Color::srgb(0., 0.22, 0.76);

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    transform: Transform,
    sprite: Sprite,
    collider: Collider,
}

#[derive(Component)]
struct ReturnButton;

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
            collider: Collider,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum BrickColor {
    Red,
    Blue,
}

pub struct RegionGamePlugin;

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

fn create_text_bundle(msg: &str, x: f32, y: f32, asset_server: &Res<AssetServer>) -> impl Bundle {
    (
        Text::new(msg),
        TextFont {
            font: asset_server.load(FIRASANS_FONT),
            font_size: 42.0,

            ..Default::default()
        },
        Transform::from_xyz(x, y, 0.),
        TextColor(GAME_DATA_TEXT_COLOR),
    )
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

const MAROON_COLOR: Color = Color::srgb(0.5019608, 0.0, 0.0);

impl Player for RedPlayer {
    const BREAK_COLOR: BrickColor = BrickColor::Red;
    const RENDER_COLOR: Color = MAROON_COLOR;
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
        let up_margin: f32 = 60.;
        let top_y: f32 = 180.;
        let x: f32 = -50.;
        commands
            .spawn((
                Text::new("RED SCORE"),
                TextFont {
                    font: asset_server.load(FIRASANS_FONT),
                    font_size: 42.0,
                    ..Default::default()
                },
                TextColor(GAME_DATA_TEXT_COLOR),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.),
                    top: Val::Px(30.0),
                    ..Default::default()
                },
                PlayBoard,
            ))
            .with_children(|parent| {
                parent
                    .spawn(create_text_bundle("0", x, top_y - up_margin, asset_server))
                    .insert(PlayerScore(Self::BREAK_COLOR));
            });
    }
    fn place_player(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        const LEN: i32 = BRICK_COUNT_WIDTH / 4;
        const RED_X: i32 = -MID_POS * BRICK_WIDTH + LEN * BRICK_WIDTH;
        use bevy::color::palettes::basic::PURPLE;
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(10.))),
            Transform {
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
            MeshMaterial2d(materials.add(Color::from(PURPLE))),
            RedPlayer::new(),
        ));
    }
}

#[derive(Component)]
struct BluePlayer {
    x: f32,
    y: f32,
}
const GRAY_COLOR: Color = Color::srgb(0.5019608, 0.5019608, 0.5019608);
impl Player for BluePlayer {
    const BREAK_COLOR: BrickColor = BrickColor::Blue;
    const RENDER_COLOR: Color = GRAY_COLOR;
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
            .spawn((Transform::from_xyz(BOARD_LEFT_POS, 0., 0.), PlayBoard, Sprite::default()))
            .with_children(|parent| {
                let up_margin: f32 = 60.;
                let top_y: f32 = 180.;
                let x: f32 = -50.;
                parent.spawn(create_text_bundle("BLUE SCORE", x, top_y, asset_server));
                parent
                    .spawn(create_text_bundle("0", x, top_y - up_margin, asset_server))
                    .insert(PlayerScore(Self::BREAK_COLOR));
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
        use bevy::color::palettes::basic::OLIVE;
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(10.))),
            Transform {
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
            MeshMaterial2d(materials.add(Color::from(Color::Srgba(OLIVE)))),
            BluePlayer::new(),
        ));
    }
}

impl BluePlayer {
    fn new() -> Self {
        Self { x: -100., y: -100. }
    }
}

impl Plugin for RegionGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::RegionGame),
            (setup_basedata, setup_player).chain(),
        )
        .add_systems(
            OnExit(GameState::RegionGame),
            (
                despawn_with_component::<Brick>,
                despawn_with_component::<Collider>,
                despawn_with_component::<PlayBoard>,
                despawn_with_component::<ReturnButton>,
                despawn_with_component::<RedPlayer>,
                despawn_with_component::<BluePlayer>,
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
                .run_if(in_state(GameState::RegionGame)),
        )
        .add_systems(
            Update,
            (common_button_system, menu_action)
                .chain()
                .run_if(in_state(GameState::RegionGame)),
        );
    }
}

fn setup_basedata(mut commands: Commands, asset_server: Res<AssetServer>) {
    for index_y in 0..BRICK_COUNT_WIDTH + 1 {
        let real_y = (index_y - MID_POS) * BRICK_WIDTH;
        for index_x in 0..BRICK_COUNT_WIDTH + 1 {
            let real_x = (index_x - MID_POS) * BRICK_WIDTH;
            commands.spawn((
                Sprite {
                    color: if real_x > 0 {
                        BluePlayer::RENDER_COLOR
                    } else {
                        RedPlayer::RENDER_COLOR
                    },
                    ..default()
                },
                Transform {
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
                Brick(if real_x > 0 {
                    BrickColor::Blue
                } else {
                    BrickColor::Red
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
    commands
        .spawn((
            BackgroundColor(NORMAL_BUTTON),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.),
                ..default()
            },
            Button,
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
            ));
            parent.spawn((
                (Text::new("GoBack"), button_text_style),
                TextColor(TEXT_COLOR),
            ));
        });
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    RedPlayer::place_player(&mut commands, &mut meshes, &mut materials);
    BluePlayer::place_player(&mut commands, &mut meshes, &mut materials);
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

// Returns `Some` if `ball` collides with `bounding_box`.
// The returned `Collision` is the side of `bounding_box` that `ball` hit.
fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset: Vec2 = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

fn check_collider<P>(
    mut blocks: Query<(&Transform, &mut Sprite, &mut Brick)>,
    walls: Query<&Transform, With<Collider>>,
    mut player: Query<(&Transform, &mut P), With<P>>,
) where
    P: Player + Component,
{
    let (player_tran, mut player_state) = player.single_mut();
    for (transform, mut spite, mut block) in &mut blocks {
        if let Some(coll) = ball_collision(
            BoundingCircle::new(player_tran.translation.truncate(), 20. / 2.0),
            Aabb2d::new(
                transform.translation.truncate(),
                transform.scale.truncate() / 2.0,
            ),
        ) {
            let x = player_state.x();
            let y = player_state.y();
            if block.0 != P::BREAK_COLOR {
                match coll {
                    Collision::Left | Collision::Right => player_state.set_x(-x),
                    Collision::Top | Collision::Bottom => player_state.set_y(-y),
                }

                spite.color = P::RENDER_COLOR;
                block.0 = P::BREAK_COLOR;
            }
        }
    }
    for transform in &walls {
        if let Some(coll) = ball_collision(
            BoundingCircle::new(player_tran.translation.truncate(), 20. / 2.0),
            Aabb2d::new(
                transform.translation.truncate(),
                transform.scale.truncate() / 2.0,
            ),
        ) {
            let x = player_state.x();
            let y = player_state.y();
            match coll {
                Collision::Left | Collision::Right => player_state.set_x(-x),
                Collision::Top | Collision::Bottom => player_state.set_y(-y),
            }
        }
    }
}

fn handle_move<P>(mut blue_query: Query<(&mut Transform, &P), With<P>>, timer: Res<Time<Fixed>>)
where
    P: Player + Component,
{
    // TODO: place the player
    let (mut player_trans, state) = blue_query.single_mut();

    player_trans.translation.x += state.x() * timer.delta().as_secs_f32();
    player_trans.translation.y += state.y() * timer.delta().as_secs_f32();
}

fn handle_score_update(
    text_query: Query<(Entity, &PlayerScore), (With<Text>, With<PlayerScore>)>,
    blocks: Query<&Brick>,
    mut writer: TextUiWriter,
) {
    for (text, playerboard) in &text_query {
        let count = blocks.iter().filter(|b| b.0 == playerboard.0).count();
        // *writer.text(text, 1) = count.to_string();
    }
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
