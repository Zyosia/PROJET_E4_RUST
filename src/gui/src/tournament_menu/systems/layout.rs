use bevy::prelude::*;

use crate::tournament_menu::components::*;
use crate::tournament_menu::styles::*;
use crate::gui_helper_functions::text_helper::*;

use tournament_engine::tournament::Tournament;

const Z_LAYER_UI: f32 = 10.0;

pub fn spawn_tournament_menu(mut commands: Commands, asset_server: Res<AssetServer>, tournament: Res<Tournament>) {
    build_tournament_menu(&mut commands, &asset_server, &tournament);
}

pub fn despawn_tournament_menu(mut commands: Commands, tournament_menu_query: Query<Entity, With<TournamentMenu>>) {
    if let Ok(tournament_menu_entity) = tournament_menu_query.get_single() {
        commands.entity(tournament_menu_entity).despawn_recursive();
    }
}

pub fn build_tournament_menu(commands: &mut Commands, asset_server: &Res<AssetServer>, tournament: &Tournament) -> Entity {
    let participants = tournament.get_participants();
    let tournament_name = tournament.get_name();

    // Define min and max font sizes
    let max_font_size = 70.0;
    let min_font_size = 24.0;
    let name_length = tournament_name.len() as f32;
    // Dynamically calculate font size based on name length
    let title_font_size = (max_font_size - name_length).clamp(min_font_size, max_font_size);

    println!("Participants: {:?}", participants); // <- Check if this prints anything

    let tournament_menu_entity = commands
    .spawn((
        TournamentMenu {},
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, Z_LAYER_UI)),
    ))
    .with_children(|parent| {
        // Tournament name
        parent.spawn((
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            Text::new(format!("Tournoi : {}", tournament_name)),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: title_font_size,
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, Z_LAYER_UI)),
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
            row.spawn(Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::FlexStart,
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            })
            .with_children(|left| {
                // Left: Insert AI
                left.spawn((
                    AiInsertMenuButton {},
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(100.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(0.0, 0.0, Z_LAYER_UI)),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                ))
                .with_child((
                    Text::new("Insérez une IA"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

                // "Changer le nombre de parties" button
                left.spawn((
                    ChangeNumberOfGameButton {},
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(100.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(0.0, 0.0, Z_LAYER_UI)),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                ))
                .with_child((
                    Text::new("Changer le \n nombre de parties"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            });

            // Center: Participant list container (scroll area)
            row.spawn(Node {
                width: Val::Px(400.0),
                height: Val::Px(500.0), // max height for the scroll container
                overflow: Overflow::clip_y(),
                flex_direction: FlexDirection::Row, // Keeping the Row layout for horizontal alignment
                ..default()
            })
            .with_children(|scroll_row| {
                // LEFT: Scrollable content (participants list)
                scroll_row.spawn((
                    ParticipantScrollArea,
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
                    for name in participants {
                        center.spawn((
                            Text::new(name),
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
                    ParticipantScrollBar,
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
                        ParticipantScrollBarHandle,
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


            // Right: Info, Result and Start buttons
            row.spawn(Node {
                margin: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|right| {

                // Tournament Info Box
                right.spawn((
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
                    Transform::from_translation(Vec3::new(0.0, 0.0, Z_LAYER_UI)),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ))
                .with_children(|info_box| {
                    let bold_font = asset_server.load("fonts/FiraSans-Bold.ttf");

                    spawn_centered_info_text(info_box, "Infos Tournoi", bold_font.clone(), 28.0, Color::srgb(1.0, 0.85, 0.4));

                    info_box.spawn(Node {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|node| {
                        node.spawn((
                            Text::new(format!("Parties par match : {}", tournament.get_number_of_games())),
                            TextFont {
                                font: bold_font.clone(),
                                font_size: 22.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            NumberOfGamesText,
                        ));
                    });

                    spawn_centered_info_text(
                        info_box,
                        &format!("Statut : {}", tournament.get_state_to_string()), // Assuming you have this too
                        bold_font.clone(),
                        22.0,
                        Color::WHITE,
                    );
                });

                // "Voir Résultats" button
                right.spawn((
                    WatchResultsMenuButton {},
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
                    Transform::from_translation(Vec3::new(0.0, 0.0, Z_LAYER_UI)),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                ))
                .with_child((
                    Text::new("Voir Résultats"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

                // "Démarrer le tournoi" button
                right.spawn((
                    StartTournamentButton {},
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
                    Transform::from_translation(Vec3::new(0.0, 0.0, Z_LAYER_UI)),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                ))
                .with_child((
                    Text::new("Démarrer le tournoi"),
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
        parent.spawn((
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, Z_LAYER_UI)),
        ))
        .with_children(|bottom| {
            bottom.spawn((
                EndTournamentButton {},
                Button,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(100.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Transform::from_translation(Vec3::new(0.0, 0.0, Z_LAYER_UI)),
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON_COLOR),
            ))
            .with_child((
                Text::new("Mettre fin au tournoi"),
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

    tournament_menu_entity
}