use bevy::prelude::*;

struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), main_menu_setup)
            .add_systems(Update, common_button_system);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ee".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, camera_setup)
        .add_state::<GameState>()
        .add_plugins(MenuPlugin)
        .run()
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    EasyPlay,
    NormalPlay,
    HardPlay,
    Help,
    Quit,
}

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
pub const BACKGROUND: Color = Color::rgb(0.27, 0.43, 0.8);

pub const BACKGROUND_COLOR: Color = Color::rgb(0.61, 0.7, 0.71);

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const GAME_DATA_TEXT_COLOR: Color = Color::rgb(0., 0.22, 0.76);

pub trait EntitySpawner {
    fn spawn_button(
        &mut self,
        bundle: impl Bundle,
        icon_image_path: &'static str,
        title: &str,
        asset_server: &Res<AssetServer>,
    );
}

impl EntitySpawner for ChildBuilder<'_, '_, '_> {
    fn spawn_button(
        &mut self,
        bundle: impl Bundle,
        icon_image_path: &'static str,
        title: &str,
        asset_server: &Res<AssetServer>,
    ) {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let button_style = Style {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_style = Style {
            width: Val::Px(30.0),
            height: Val::Auto,
            position_type: PositionType::Relative,
            ..default()
        };
        let button_text_style = TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: TEXT_COLOR,
        };

        self.spawn((
            ButtonBundle {
                style: button_style,
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            bundle,
        ))
        .with_children(|parent| {
            let icon = asset_server.load(icon_image_path);
            parent.spawn(ImageBundle {
                style: button_icon_style,
                image: UiImage::new(icon),
                ..default()
            });
            parent.spawn(TextBundle::from_section(title, button_text_style));
        });
    }
}

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

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
pub fn common_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}
