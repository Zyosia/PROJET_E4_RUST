use bevy::prelude::*;
use bevy::color::palettes::basic::*;
use bevy::window::PrimaryWindow;

use crate::ai_insert_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::ai_insert_menu::components::*;
use crate::AppState;
use crate::IsPopup;

use crate::gui_helper_functions::windows::pop_up::request::request::spawn_request_popup;
use crate::gui_helper_functions::windows::pop_up::request::components::PopupPurpose;

pub fn interact_with_browser_button(
    mut is_popup: ResMut<IsPopup>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<BrowserButton>)
    >,
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();

                if is_popup.0 == false {
                    is_popup.0 = true;
                    spawn_request_popup("Entrez le nom de votre IA:", &mut commands, &asset_server, windows, PopupPurpose::SetAiName);
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