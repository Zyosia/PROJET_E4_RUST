mod main_menu;
mod tournament_menu;
mod tournament_result_menu;
mod ai_insert_menu;
mod systems;
mod gui_helper_functions;

use crate::gui_helper_functions::windows::pop_up::loading::interactions::*;

use main_menu::MainMenuPlugin;
use tournament_menu::TournamentMenuPlugin;
use tournament_result_menu::TournamentResultMenuPlugin;
use ai_insert_menu::AiInsertMenuPlugin;

use bevy::{prelude::*, winit::WinitSettings};
use bevy_simple_text_input::TextInputPlugin;

use systems::*;

use tournament_engine::tournament::Tournament;

pub fn run_graphics() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(TextInputPlugin)
        .init_state::<AppState>()
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(Tournament::new())
        .insert_resource(ScrollDragState::default())
        .insert_resource(IsPopup::default())
        .insert_resource(LoadingTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        })
        .add_event::<ResetTournamentEvent>()
        .add_systems(Startup, setup)
        // My Plugins
        .add_plugins((MainMenuPlugin, TournamentMenuPlugin, TournamentResultMenuPlugin, AiInsertMenuPlugin))
        //Custom Systems
        .add_systems(OnEnter(AppState::CreatingTournament), create_tournament_system)
        .add_systems(OnEnter(AppState::PlayingTournament), playing_tournament_system)
        //Update Systems
        .add_systems(
            Update,
            (
                roll_circle_system,
                animate_loading_text_system.run_if(
                    in_state(AppState::CreatingTournament)
                    .or(in_state(AppState::PlayingTournament))
                ),
                exit_app,
                reset_tournament_on_event,
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    TournamentMenu,
    TournamentResultMenu,
    AiInsertMenu,
    WatchMatchMenu,
    CreatingTournament,
    PlayingTournament,
}

#[derive(Resource, Default)]
pub struct ScrollDragState {
    pub dragging: bool,
    pub start_cursor_y: f32,
    pub start_scroll_y: f32,
}

/// Detect wether there is a popup or not on the window
#[derive(Resource, Default)]
pub struct IsPopup(pub bool);

#[derive(Resource)]
pub struct PendingTournamentName(String);

#[derive(Resource, Default)]
pub struct LoadingTimer {
    pub timer: Timer,
}