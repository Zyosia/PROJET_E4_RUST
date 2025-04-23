mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use bevy::prelude::*;

use crate::AppState;

pub struct TournamentMenuPlugin;

impl Plugin for TournamentMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::TournamentMenu), spawn_tournament_menu)
            // Systems
            .add_systems(
                Update,
                (interact_with_end_tournament_button, interact_with_ai_insert_menu_button, interact_with_start_tournament_button)
                    .run_if(in_state(AppState::TournamentMenu)),
            )
            // OnExit State Systems
            .add_systems(OnExit(AppState::TournamentMenu), despawn_tournament_menu);
    }
}