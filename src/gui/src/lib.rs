mod main_menu;
mod ai_insert_menu;
mod systems;

use main_menu::MainMenuPlugin;
use ai_insert_menu::AiInsertMenuPlugin;

use bevy::{prelude::*, winit::WinitSettings};
use systems::*;

pub fn run_graphics() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        // My Plugins
        .add_plugins((MainMenuPlugin, AiInsertMenuPlugin))
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

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    AiInsertMenu,
    WatchMatchMenu,
}