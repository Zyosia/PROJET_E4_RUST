use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::window::PrimaryWindow;

use crate::gui_helper_functions::windows::pop_up::choice::components::*;

pub fn despawn_choice_popup(
    commands: &mut Commands,
    popup_query: Query<Entity, With<ChoicePopup>>,
    overlay_query: Query<Entity, With<ChoicePopupOverlay>>,
){
    if let Ok(popup_entity) = popup_query.get_single() {
        if commands.get_entity(popup_entity).is_some() {
            commands.entity(popup_entity).despawn_recursive();
        }
    }                                
    if let Ok(overlay_entity) = overlay_query.get_single() {
        if commands.get_entity(overlay_entity).is_some() {
            commands.entity(overlay_entity).despawn_recursive();
        }
    } 
}

pub fn spawn_choice_popup(
    message: &str,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    popup_purpose:  ChoicePopupPurpose,
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
        ChoicePopupOverlay,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        FocusPolicy::Block,
        ZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 900.0)), // overlay behind popup
    ));

    commands.spawn((
        ChoicePopup,
        popup_purpose,
        Node {
            width: Val::Px(popup_width),
            height: Val::Px(popup_height),
            position_type: PositionType::Absolute,
            left: Val::Px(center_x),
            top: Val::Px(center_y),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        ZIndex(2),
        BorderColor(Color::WHITE),
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)), // popup in front
    )).with_children(|popup| {
        popup.spawn((
            Text::new(message),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));

        popup.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                width: Val::Percent(100.0),
                ..default()
            },
        )).with_children(|row| {
            row.spawn((
                YesButton,
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::srgb(0.0, 0.5, 0.0)),
                BackgroundColor(Color::srgb(0.0, 0.3, 0.0)),
            )).with_child((
                Text::new("Oui"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            row.spawn((
                NoButton,
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::srgb(0.6, 0.0, 0.0)),
                BackgroundColor(Color::srgb(0.4, 0.0, 0.0)),
            )).with_child((
                Text::new("Non"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}
