use bevy::prelude::*;

use crate::gui_helper_functions::windows::pop_up::components::*;

pub fn spawn_number_of_games_popup(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        NumberOfGamesPopup,
        Node {
            width: Val::Px(400.0),
            height: Val::Px(250.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-200.0, -125.0, 100.0)),
        BorderColor(Color::WHITE),
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    ))
    .with_children(|popup| {
        popup.spawn((
            Text::new("Entrez le nombre de parties:"),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
        
        //InputBoxField here
        /*
        popup.spawn((
            NumberOfGamesInputField,
            TextInput::default(), // Assuming you have a component to handle text input
            Node {
                width: Val::Px(200.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(Color::BLACK),
        ));
        */

        // Confirm and Cancel buttons
        popup.spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceAround,
            width: Val::Percent(100.0),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                ConfirmNumberOfGamesButton,
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

            row.spawn((
                CancelNumberOfGamesButton,
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::srgb(0.6, 0.0, 0.0)), // Dark red
                BackgroundColor(Color::srgb(0.4, 0.0, 0.0)),
            ))
            .with_child((
                Text::new("Annuler"),
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
