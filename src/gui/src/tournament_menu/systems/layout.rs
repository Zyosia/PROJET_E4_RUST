use bevy::prelude::*;

use crate::tournament_menu::components::*;
use crate::tournament_menu::styles::*;

pub fn spawn_tournament_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_tournament_menu(&mut commands, &asset_server);
}

pub fn despawn_tournament_menu(mut commands: Commands, tournament_menu_query: Query<Entity, With<TournamentMenu>>) {
    println!("test_despawn");
    if let Ok(tournament_menu_entity) = tournament_menu_query.get_single() {
        commands.entity(tournament_menu_entity).despawn_recursive();
    }
}

pub fn build_tournament_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let tournament_menu_entity = commands
        .spawn((
            TournamentMenu {},
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }
        ))
        .with_children(|parent| {
            // Bouton "Insérez une IA"
            parent
                .spawn((
                    AiInsertMenuButton {},
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
                    Text::new("Insérez une IA"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            parent
                .spawn((
                    WatchMatchMenuButton {},
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
                        top: Val::Percent(60.0),
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                ))
                .with_child((
                    Text::new("Voir les matchs"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            parent
                .spawn((
                    EndTournamentButton {},
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
                    Text::new("Quitter"),
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
                    Text::new("Tournament :"),
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
        tournament_menu_entity
    }