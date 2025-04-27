use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use winit::dpi::{PhysicalPosition, Position};

use crate::IsPopup;
use crate::{AppState, PendingTournamentName};
use crate::gui_helper_functions::windows::pop_up::loading::loading::despawn_loading_popup;
use crate::gui_helper_functions::windows::pop_up::loading::components::{LoadingPopup, LoadingPopupOverlay};

use tournament_engine::tournament::Tournament;


pub fn setup(
    mut commands: Commands,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
) {
    // ui camera
    commands.spawn(Camera2d);
    focus_window(primary_window_query, winit_windows)
}

fn focus_window(
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
) {
    if let Ok(primary_entity) = primary_window_query.get_single() {
        if let Some(winit_window) = winit_windows.get_window(primary_entity) {
            winit_window.focus_window(); // ← THIS works

            // Center the window
            if let Some(monitor) = winit_window.current_monitor() {
                let monitor_size = monitor.size();
                let window_size = winit_window.outer_size();

                let new_x = ((monitor_size.width as f64 - window_size.width as f64) / 2.0).round() as i32;
                let new_y = ((monitor_size.height as f64 - window_size.height as f64) / 2.0).round() as i32;

                winit_window.set_outer_position(
                    Position::Physical(PhysicalPosition::new(new_x, new_y))
                );
            }
        }
    }
}

pub fn exit_app(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}

pub fn reset_tournament_on_event(
    mut tournament: ResMut<Tournament>,
    mut events: EventReader<ResetTournamentEvent>,
) {
    for _ in events.read() {
        tournament.reset();
    }
}

pub fn create_tournament_system(
    mut commands: Commands,
    mut tournament: ResMut<Tournament>,
    mut is_popup: ResMut<IsPopup>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    popup_query: Query<Entity, With<LoadingPopup>>,
    overlay_query: Query<Entity, With<LoadingPopupOverlay>>,
    pending_name: Option<Res<PendingTournamentName>>,
) {
    if let Some(pending_name) = pending_name {
        println!("Creating tournament with name: {}", pending_name.0);

        tournament.set_tournament_name(pending_name.0.clone()).unwrap();
        match tournament.add_all_ai() {
            Ok(()) => println!("✅ AI compiled and added successfully!"),
            Err(e) => eprintln!("❌ Failed to compile/add AI: {}", e),
        }

        // Despawn loading popup
        despawn_loading_popup(&mut commands, popup_query, overlay_query);

        // Clear pending tournament name
        commands.remove_resource::<PendingTournamentName>();

        // Close popup
        is_popup.0 = false;

        // Move to tournament menu
        app_state_next_state.set(AppState::TournamentMenu);
    }
}

pub fn playing_tournament_system(
    mut commands: Commands,
    mut tournament: ResMut<Tournament>,
    mut is_popup: ResMut<IsPopup>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    popup_query: Query<Entity, With<LoadingPopup>>,
    overlay_query: Query<Entity, With<LoadingPopupOverlay>>,
) {
        
    tournament.start().unwrap();
    tournament.run().unwrap();

    // Despawn loading popup
    despawn_loading_popup(&mut commands, popup_query, overlay_query);

    // Close popup
    is_popup.0 = false;

    // Move to tournament menu
    app_state_next_state.set(AppState::TournamentResultMenu);
}

#[derive(Event)]
pub struct ResetTournamentEvent;