pub mod components;
mod styles;
mod systems;

use systems::interactions::*;
use crate::gui_helper_functions::windows::pop_up::request::interactions::*;
use crate::gui_helper_functions::windows::pop_up::information::interactions::*;
use crate::gui_helper_functions::windows::pop_up::choice::interactions::*;
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
                (
                    start_participant_scroll_drag, 
                    update_participant_scroll_drag,
                    cancel_scroll_drag_on_cursor_leave,
                    scroll_participant_list, 
                    update_scroll_bar, 
                    interact_with_end_tournament_button, 
                    interact_with_ai_insert_menu_button,
                    interact_with_watch_results_button,
                    interact_with_change_number_of_game_button,
                    interact_with_start_tournament_button,

                    //pop_up interact
                    interact_with_cancel_request_pop_up_button,
                    interact_with_confirm_request_pop_up_button,
                    interact_with_ok_information_pop_up_button,
                    interact_with_no_pop_up_button,
                    interact_with_yes_pop_up_button,
                )
                    .run_if(in_state(AppState::TournamentMenu)),
            )
            // OnExit State Systems
            .add_systems(OnExit(AppState::TournamentMenu), despawn_tournament_menu);
    }
}