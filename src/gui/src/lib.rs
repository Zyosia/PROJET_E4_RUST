mod main_menu;
mod tournament_menu;
mod ai_insert_menu;
mod systems;

use main_menu::MainMenuPlugin;
use tournament_menu::TournamentMenuPlugin;
use ai_insert_menu::AiInsertMenuPlugin;

use bevy::{prelude::*, winit::WinitSettings};
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use winit::dpi::{PhysicalPosition, Position};

use systems::*;

use tournament_engine::Tournament;

pub fn run_graphics() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(Tournament::new())
        .insert_resource(ScrollDragState::default())
        .add_systems(Startup, (setup, focus_window))
        // My Plugins
        .add_plugins((MainMenuPlugin, TournamentMenuPlugin, AiInsertMenuPlugin))
        .add_systems(
            Update,
            (
                // transition_to_ai_insert_menu_state,
                // transition_to_watch_match_menu_state,
                // transition_to_main_menu_state,
                exit_app,
            ),
        )
        .run();
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

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    TournamentMenu,
    AiInsertMenu,
    WatchMatchMenu,
}

#[derive(Resource, Default)]
pub struct ScrollDragState {
    pub dragging: bool,
    pub start_cursor_y: f32,
    pub start_scroll_y: f32,
}
