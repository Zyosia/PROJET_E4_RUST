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
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            AiInsertMenu {}
        ))
        .with_children(|parent| {
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
                    Text::new("Parcourir les fichiers locaux"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            parent
                .spawn((
                    PathAIText {},
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(5.0), 
                        ..default()
                    },
                    Text::new(""),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 90.0,
                        ..default()
                    },
                    // Because this is a distinct label widget and
                    // not button/list item text, this is necessary
                    // for accessibility to treat the text accordingly.
                    Label,
                ));
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
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(5.0), 
                        ..default()
                    },
                    Text::new("Ajouter une IA au tournoi"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 90.0,
                        ..default()
                    },
                    // Because this is a distinct label widget and
                    // not button/list item text, this is necessary
                    // for accessibility to treat the text accordingly.
                    Label,
                ));
        })
        .id();
    insert_ai_menu_entity
}
            
        