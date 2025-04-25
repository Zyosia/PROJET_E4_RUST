use bevy::prelude::*;

use crate::ai_insert_menu::components::*;
use crate::ai_insert_menu::styles::*;

pub fn spawn_ai_insert_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_insert_ai_menu(&mut commands, &asset_server);
}

pub fn despawn_ai_insert_menu(mut commands: Commands, insert_ai_menu_query: Query<Entity, With<AiInsertMenu>>) {
    for entity in insert_ai_menu_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
}

pub fn build_insert_ai_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let insert_ai_menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            AiInsertMenu {}
        ))
        .with_children(|parent| {

            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(5.0), 
                        ..default()
                    },
                    Text::new("Menu d'ajout d'IA"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 90.0,
                        ..default()
                    },
                ));
            // Bouton "Parcourir les fichiers locaux"
            parent
                .spawn((
                    BrowserButton {},
                    Button,
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        top: Val::Percent(35.0),
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                ))
                .with_child((
                    Text::new("Ajouter une nouvelle IA"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            // Nom IA ajouté/Confirmation 
            parent
                .spawn((
                    PathAIText,
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(55.0),
                        left: Val::Percent(25.0),
                        width: Val::Percent(50.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    BorderRadius::MAX,
                    BorderColor(Color::BLACK),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Text::new("Path to your AI file"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            parent
                .spawn((
                    ReturnMainMenuButton {},
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(70.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        top: Val::Percent(85.0),
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                ))
                .with_child((
                    Text::new("Retour"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        })
        .id();
    insert_ai_menu_entity
}
            
        