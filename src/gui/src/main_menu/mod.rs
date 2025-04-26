mod components;
mod styles;
mod systems;

use systems::interactions::*;
use crate::gui_helper_functions::windows::pop_up::request::interactions::*;
use crate::gui_helper_functions::windows::pop_up::choice::interactions::*;

use systems::layout::*;

use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;


impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            // Systems
            .add_systems(
                Update,
                (
                    interact_with_quit_button, 
                    interact_with_start_tournament_button,

                    //popup_interactions
                    interact_with_cancel_request_pop_up_button,
                    interact_with_confirm_request_pop_up_button,
                    interact_with_no_pop_up_button,
                    interact_with_yes_pop_up_button,
                )
                .run_if(in_state(AppState::MainMenu)),
            )
            // OnExit State Systems
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}