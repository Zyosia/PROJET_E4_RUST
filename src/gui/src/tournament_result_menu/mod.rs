mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use bevy::prelude::*;

use crate::AppState;

pub struct TournamentResultMenuPlugin;

impl Plugin for TournamentResultMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::TournamentResultMenu), spawn_tournament_result_menu)
            // Systems
            .add_systems(
                Update,
                (
                    start_scoreboard_scroll_drag, 
                    update_scoreboard_scroll_drag,
                    cancel_scroll_drag_on_cursor_leave,
                    scroll_scoreboard_list, 
                    update_scroll_bar, 
                    interact_with_return_button,
                )
                    .run_if(in_state(AppState::TournamentResultMenu)),
            )
            // OnExit State Systems
            .add_systems(OnExit(AppState::TournamentResultMenu), despawn_tournament_result_menu);
    }
}