use bevy::prelude::*;
use bevy::color::palettes::basic::*;

use crate::main_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::main_menu::components::*;
use crate::AppState;

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<QuitButton>)
    >
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();
                app_exit_event_writer.send(AppExit::Success);
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
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<CreateTournamentButton>)
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();
                
                app_state_next_state.set(AppState::TournamentMenu);
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