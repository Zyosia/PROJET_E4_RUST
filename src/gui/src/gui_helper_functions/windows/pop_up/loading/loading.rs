use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::ui::FocusPolicy;

use crate::gui_helper_functions::windows::pop_up::loading::components::*;

pub fn despawn_loading_popup(
    commands: &mut Commands,
    popup_query: Query<Entity, With<LoadingPopup>>,
    overlay_query: Query<Entity, With<LoadingPopupOverlay>>,
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

pub fn spawn_loading_popup(
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
        LoadingPopupOverlay,
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
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)), // Semi-transparent black
        Transform::from_translation(Vec3::new(0.0, 0.0, 999.0)), // Behind popup, in front of everything else
    ));

    commands.spawn((
        LoadingPopup,
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
        ZIndex(2),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
        BorderColor(Color::WHITE),
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    ))
    .with_children(|popup| {
        popup.spawn((
            LoadingText,
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
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)), // Red color
            BorderRadius::MAX, // Make it a circle
            RollAnimation,
        ));

    });
}