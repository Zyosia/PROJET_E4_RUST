use bevy::prelude::*;

use crate::tournament_result_menu::components::*;
use crate::tournament_result_menu::styles::*;
use crate::gui_helper_functions::text_helper::*;

use tournament_engine::Tournament;

pub fn spawn_tournament_result_menu(mut commands: Commands, asset_server: Res<AssetServer>, tournament: Res<Tournament>) {
    build_tournament_result_menu(&mut commands, &asset_server, &tournament);
}

pub fn despawn_tournament_result_menu(mut commands: Commands, tournament_result_menu_query: Query<Entity, With<TournamentResultMenu>>) {
    if let Ok(tournament_result_menu_entity) = tournament_result_menu_query.get_single() {
        commands.entity(tournament_result_menu_entity).despawn_recursive();
    }
}

pub fn build_tournament_result_menu(commands: &mut Commands, asset_server: &Res<AssetServer>, tournament: &Tournament) -> Entity {
    let scoreboard = tournament.get_all_scores();
    let mut sorted_scores: Vec<_> = scoreboard.iter().collect();
    sorted_scores.sort_by(|a, b| b.1.cmp(a.1)); // Sort descending by score

    let tournament_name = tournament.get_name();
    // Define min and max font sizes
    let max_font_size = 70.0;
    let min_font_size = 24.0;
    let name_length = tournament_name.len() as f32;
    // Dynamically calculate font size based on name length
    let title_font_size = (max_font_size - name_length).clamp(min_font_size, max_font_size);
    
    let tournament_result_menu_entity = commands
    .spawn((
        TournamentResultMenu {},
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
    ))
    .with_children(|parent| {
        // Tournament name
        parent.spawn((
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            Text::new(format!("Résultats du Tournoi : {}", tournament_name)),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: title_font_size,
                ..default()
            },
            Label,
        ));

        // Main content: Row layout
        parent.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(70.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::FlexStart,
            ..default()
        }).with_children(|row| {
            // Left: Tournament Informations
            row.spawn(Node {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            })
            .with_children(|left| {
                left.spawn((
                    Node {
                        width: Val::Auto,
                        min_width: Val::Px(300.0),
                        max_width: Val::Percent(90.0),
                        height: Val::Auto,
                        margin: UiRect::all(Val::Px(10.0)),
                        padding: UiRect::all(Val::Px(15.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ))
                .with_children(|info_box| {

                    let bold_font = asset_server.load("fonts/FiraSans-Bold.ttf");

                    spawn_centered_info_text(info_box, "Infos Tournoi", bold_font.clone(), 28.0, Color::srgb(1.0, 0.85, 0.4));

                    spawn_centered_info_text(
                        info_box,
                        &format!("Parties par match : {}", tournament.get_number_of_games()),
                        bold_font.clone(),
                        24.0,
                        Color::srgb(0.9, 0.9, 0.9),
                    );

                    spawn_centered_info_text(
                        info_box,
                        &format!("IA participantes : {}", tournament.get_participants().len()),
                        bold_font.clone(),
                        24.0,
                        Color::srgb(0.9, 0.9, 0.9),
                    );

                    spawn_centered_info_text(
                        info_box,
                        &format!("Nombre totales\nde parties : {}", tournament.get_total_number_of_games()),
                        bold_font.clone(),
                        24.0,
                        Color::srgb(0.9, 0.9, 0.9),
                    );

                });
                
            });

            // Center: Scorboard list container (scroll area)
            row.spawn(Node {
                width: Val::Px(400.0),
                height: Val::Px(500.0), // max height for the scroll container
                overflow: Overflow::clip_y(),
                flex_direction: FlexDirection::Row, // Keeping the Row layout for horizontal alignment
                ..default()
            })
            .with_children(|scroll_row| {
                // LEFT: Scrollable content (Scorboard list)
                scroll_row.spawn((
                    ScoreBoardScrollArea,
                    Node {
                        width: Val::Percent(95.0), // Make the participant list take 95% of the space
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        margin: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        top: Val::Px(0.0),
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    Transform::default(),
                    GlobalTransform::default(),
                ))
                .with_children(|center| {
                    center.spawn((
                        Text::new("Participants :"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                    for (ai, score) in sorted_scores {
                        center.spawn((
                            Text::new(format!("{} → {} points", ai, score)),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));
                    }
                });
            
                // RIGHT: Scroll bar track
                scroll_row.spawn((
                    ScoreBoardScrollBar,
                    Node {
                        width: Val::Percent(5.0), // This will make the scrollbar take 5% of the space
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        top: Val::Px(0.0),
                        position_type: PositionType::Relative, // Absolute positioning for the scrollbar
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                ))
                .with_children(|bar| {
                    bar.spawn((
                        ScoreBoardScrollBarHandle,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(100.0), // Will be updated dynamically
                            top: Val::Px(0.0),
                            position_type: PositionType::Absolute, // Absolute position to ensure it starts at the top
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                });
            });                        

            // Right: Watch button
            row.spawn(Node {
                margin: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|right| {
                // "Voir les matchs" button
                right.spawn((
                    WatchMatchMenuButton {},
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(80.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
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
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            });
        });

        // Bottom: End tournament button
        parent.spawn(Node {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        })
        .with_children(|bottom| {
            bottom.spawn((
                ReturnButton {},
                Button,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(100.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
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
        });
    })
    .id();

    tournament_result_menu_entity
}