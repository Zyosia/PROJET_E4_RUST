use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::gui_helper_functions::windows::pop_up::information::components::*;

pub fn spawn_information_popup(
    message: &str,
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {

    let window = match windows.get_single() {
        Ok(win) => win,
        Err(_) => return, // If no window, just skip
    };

    let popup_width = 400.0;
    let popup_height = 250.0;

    let center_x = window.width() / 2.0 - popup_width / 2.0;
    let center_y = window.height() / 2.0 - popup_height / 2.0;

    commands.spawn((
        InformationPopup,
        Node {
            width: Val::Px(popup_width),
            height: Val::Px(popup_height),
            position_type: PositionType::Absolute,
            left: Val::Px(center_x),
            top: Val::Px(center_y),
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
        BorderColor(Color::WHITE),
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    ))
    .with_children(|popup| {
        popup.spawn((
            Text::new(message),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));

        // ok button
        popup.spawn((
            OkInformationButton,
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::srgb(0.0, 0.5, 0.0)), // Dark green
            BackgroundColor(Color::srgb(0.0, 0.3, 0.0)),
        ))
        .with_child((
            Text::new("OK"),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}
