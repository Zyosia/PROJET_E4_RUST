mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use bevy::prelude::*;

use crate::AppState;

pub struct AiInsertMenuPlugin;

impl Plugin for AiInsertMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::AiInsertMenu), spawn_ai_insert_menu)
            // Systems
            .add_systems(
                Update, 
                (interact_with_browser_button, interact_with_return_main_menu_button)
                    .run_if(in_state(AppState::AiInsertMenu)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::AiInsertMenu), despawn_ai_insert_menu);
    }
}