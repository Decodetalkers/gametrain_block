use crate::common::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
#[allow(clippy::type_complexity)]
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
            height: Val::Px(95.0),
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

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_with_component<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
