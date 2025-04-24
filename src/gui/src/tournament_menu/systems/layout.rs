use bevy::prelude::*;

use crate::tournament_menu::components::*;
use crate::tournament_menu::styles::*;

use tournament_engine::Tournament;

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
                font_size: 70.0,
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
            // Left: Insert AI
            row.spawn(Node {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            })
            .with_children(|left| {
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
                        top: Val::Px(0.0),
                        ..default()
                    },
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
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2).into()),
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


            // Right: Watch and Start buttons
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
        parent.spawn(Node {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        })
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