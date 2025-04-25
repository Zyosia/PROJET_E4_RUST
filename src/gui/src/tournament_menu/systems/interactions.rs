use bevy::prelude::*;
use bevy::color::palettes::basic::*;
use bevy::input::{ButtonState, mouse::{MouseWheel, MouseButton, MouseButtonInput}};
use bevy::window::PrimaryWindow;

use crate::tournament_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::tournament_menu::components::*;
use crate::AppState;
use crate::IsPopup;
use crate::ScrollDragState;
use crate::gui_helper_functions::windows::pop_up::request::request::spawn_request_popup;
use crate::gui_helper_functions::windows::pop_up::information::information::spawn_information_popup;

use tournament_engine::Tournament;
use tournament_engine::TournamentState;

pub fn interact_with_end_tournament_button(
    is_popup: Res<IsPopup>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut tournament: ResMut<Tournament>,
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<EndTournamentButton>)
    >,
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();
                if is_popup.0 == false {
                    tournament.reset();
                    app_state_next_state.set(AppState::MainMenu);
                }

            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn interact_with_change_number_of_game_button(
    mut is_popup: ResMut<IsPopup>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut button_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<ChangeNumberOfGameButton>)
    >,
    windows: Query<&Window, With<PrimaryWindow>>,
    tournament: Res<Tournament>,
    ) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();
                if tournament.get_state() == TournamentState::Waiting && is_popup.0 == false {
                    is_popup.0 = true;
                    spawn_request_popup("Entrez le nombre de parties:", &mut commands, &asset_server, windows);
                } else if is_popup.0 == false {
                    is_popup.0 = true;
                    spawn_information_popup("Action impossible: Tournoi déjà démarré !", &mut commands, &asset_server, windows);
                }

            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn interact_with_start_tournament_button(
    mut is_popup: ResMut<IsPopup>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<StartTournamentButton>)
    >,
    mut tournament: ResMut<Tournament>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();
                if tournament.get_state() == TournamentState::Waiting{
                    tournament.start().unwrap();
                    tournament.run().unwrap();
                    app_state_next_state.set(AppState::TournamentResultMenu);
                } else if is_popup.0 == false {
                    is_popup.0 = true;
                    spawn_information_popup("Action impossible: Tournoi déjà démarré !", &mut commands, &asset_server, windows);
                }

            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn interact_with_watch_results_button(
    mut is_popup: ResMut<IsPopup>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<WatchResultsMenuButton>)
    >,
    tournament: Res<Tournament>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();
                
                if tournament.get_state() == TournamentState::Started{
                    app_state_next_state.set(AppState::TournamentResultMenu);
                }else if is_popup.0 == false {
                    is_popup.0 = true;
                    spawn_information_popup("Action impossible: Tournoi non démarré !", &mut commands, &asset_server, windows);
                }

            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn interact_with_ai_insert_menu_button(
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<AiInsertMenuButton>)
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();
                
                app_state_next_state.set(AppState::AiInsertMenu);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

const ENTRY_HEIGHT: f32 = 35.0; // Adjust if necessary
const CONTAINER_HEIGHT: f32 = 500.0;

pub fn scroll_participant_list(
    mut query: Query<(&mut Node, &Children), With<ParticipantScrollArea>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {

    let mut scroll_delta = 0.0;

    for ev in scroll_evr.read() {
        scroll_delta += ev.y * 30.0;
    }

    if scroll_delta == 0.0 {
        return;
    }

    if let Ok((mut node, children)) = query.get_single_mut() {
        let total_height = children.len() as f32 * ENTRY_HEIGHT;
        let min_y = -(total_height - CONTAINER_HEIGHT).max(0.0);
        let max_y = 0.0;

        let current_y = match node.top {
            Val::Px(y) => y,
            _ => 0.0,
        };
        let new_y = (current_y + scroll_delta).clamp(min_y, max_y);

        node.top = Val::Px(new_y);
    }
}

pub fn update_scroll_bar(
    scroll_query: Query<(&Node, &Children), With<ParticipantScrollArea>>,
    mut bar_query: Query<&mut Node, (With<ParticipantScrollBarHandle>, Without<ParticipantScrollArea>)>,
) {
    if let Ok((scroll_node, children)) = scroll_query.get_single() {
        let total_height = children.len() as f32 * ENTRY_HEIGHT;
        let scroll_pos = match scroll_node.top {
            Val::Px(y) => y,
            _ => 0.0,
        };

        let scroll_ratio = (scroll_pos / -(total_height - CONTAINER_HEIGHT).max(1.0)).clamp(0.0, 1.0);
        let handle_height = (CONTAINER_HEIGHT / total_height).clamp(0.05, 1.0) * CONTAINER_HEIGHT;

        if let Ok(mut node) = bar_query.get_single_mut() {
            node.height = Val::Px(handle_height);
            node.margin = UiRect {
                top: Val::Px((CONTAINER_HEIGHT - handle_height) * scroll_ratio),
                ..Default::default()
            };
        }
    }
}

pub fn start_participant_scroll_drag(
    mut state: ResMut<ScrollDragState>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    windows: Query<&Window, With<PrimaryWindow>>,
    q_handle: Query<(&GlobalTransform, &Node), With<ParticipantScrollBarHandle>>,
    q_scroll: Query<&Node, With<ParticipantScrollArea>>,
) {
    let Ok(window) = windows.get_single() else { return };
    let Some(cursor_pos) = window.cursor_position() else { return };

    for ev in mouse_button_events.read() {
        if ev.button == MouseButton::Left && ev.state == ButtonState::Pressed {
            if let Ok((handle_transform, handle_node)) = q_handle.get_single() {
                let handle_pos = handle_transform.translation();
                let handle_height = match handle_node.height {
                    Val::Px(height) => height,
                    _ => return, // exit if height isn't in pixels
                };
            
                let handle_top = handle_pos.y - handle_height / 2.0;
            
                if cursor_pos.y >= handle_top && cursor_pos.y <= handle_top + handle_height {
                    state.dragging = true;
                    state.start_cursor_y = cursor_pos.y;
            
                    if let Ok(scroll_node) = q_scroll.get_single() {
                        if let Val::Px(top) = scroll_node.top {
                            state.start_scroll_y = -top;
                        }
                    }
                }
            }           
        }
        if ev.button == MouseButton::Left && ev.state == ButtonState::Released {
            state.dragging = false;
        }
    }
}

pub fn update_participant_scroll_drag(
    state: Res<ScrollDragState>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut q_scroll: Query<(&mut Node, &Children), With<ParticipantScrollArea>>,
) {
    if !state.dragging {
        return;
    }

    for ev in cursor_moved_events.read() {
        let dy = state.start_cursor_y - ev.position.y;
        if let Ok((mut scroll_node, children)) = q_scroll.get_single_mut() {
            let mut new_top = state.start_scroll_y - dy;

            // Calculate total height of scroll content
            let total_height = children.len() as f32 * ENTRY_HEIGHT;
            let max_scroll = (total_height - CONTAINER_HEIGHT).max(0.0);
            new_top = new_top.clamp(0.0, max_scroll);

            // Update the top offset of the scrollable Node
            scroll_node.top = Val::Px(-new_top);
        }
    }
}

pub fn cancel_scroll_drag_on_cursor_leave(
    mut state: ResMut<ScrollDragState>,
    mut cursor_left_events: EventReader<CursorLeft>,
) {
    for _ in cursor_left_events.read() {
        state.dragging = false;
    }
}