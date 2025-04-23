use bevy::prelude::*;
use bevy::color::palettes::basic::*;


use crate::ai_insert_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::ai_insert_menu::components::*;
use crate::AppState;

use rfd::FileDialog;

pub fn interact_with_browser_button(
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<BrowserButton>)
    >,
    //mut path_ai_text_query: Query<&mut Text, With<PathAIText>>
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();
                if let Some(path) = FileDialog::new().pick_file() {
                    println!("Fichier sélectionné : {:?}", path);
                    // if let Ok(mut text) = path_ai_text_query.get_single_mut() {
                    //     text.sections[0].value = format!("{}", path.display());
                    // }
                }
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

pub fn interact_with_return_main_menu_button(
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<ReturnMainMenuButton>)
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