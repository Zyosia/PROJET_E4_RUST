use bevy::prelude::*;

use crate::tournament_menu::components::*;
use crate::tournament_menu::styles::*;

use tournament_engine::Tournament;

pub fn spawn_tournament_menu(mut commands: Commands, asset_server: Res<AssetServer>, tournament: Res<Tournament>) {
    build_tournament_menu(&mut commands, &asset_server, &tournament.get_name());
}

pub fn despawn_tournament_menu(mut commands: Commands, tournament_menu_query: Query<Entity, With<TournamentMenu>>) {
    println!("test_despawn");
    if let Ok(tournament_menu_entity) = tournament_menu_query.get_single() {
        commands.entity(tournament_menu_entity).despawn_recursive();
    }
}

pub fn build_tournament_menu(commands: &mut Commands, asset_server: &Res<AssetServer>, tournament_name: &String) -> Entity {
    let tournament_menu_entity = commands
    .spawn((
        TournamentMenu {},
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column, // <- Make children arranged vertically
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    ))
    .with_children(|parent| {
        // Button "Insérez une IA"
        parent
            .spawn((
                AiInsertMenuButton {},
                Button,
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(100.0),
                    margin: UiRect::all(Val::Px(10.0)), // <- Add spacing
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
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

        // Button "Démarrer le tournoi"
        parent
            .spawn((
                StartTournamentButton {},
                Button,
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(100.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON_COLOR),
            ))
            .with_child((
                Text::new("Démarrer le tournoi"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

        // Button "Voir les matchs"
        parent
            .spawn((
                WatchMatchMenuButton {},
                Button,
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(100.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
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

        // Button "Quitter"
        parent
            .spawn((
                EndTournamentButton {},
                Button,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(100.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON_COLOR),
            ))
            .with_child((
                Text::new("Mettre fin au tournois"),
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
                Text::new(format!("Tournois : {}", tournament_name)),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 70.0,
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