use bevy::prelude::*;
use bevy::color::palettes::basic::*;
use bevy::window::PrimaryWindow;

use crate::main_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::main_menu::components::*;
use crate::IsPopup;

use crate::gui_helper_functions::windows::pop_up::request::request::spawn_request_popup;
use crate::gui_helper_functions::windows::pop_up::request::components::PopupPurpose;

use crate::gui_helper_functions::windows::pop_up::choice::choice::spawn_choice_popup;
use crate::gui_helper_functions::windows::pop_up::choice::components::ChoicePopupPurpose;

pub fn interact_with_quit_button(
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
        (Changed<Interaction>, With<QuitButton>)
    >
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();
                if is_popup.0 == false {
                    is_popup.0 = true;
                    spawn_choice_popup("Quitter l'application ?", &mut commands, &asset_server, windows, ChoicePopupPurpose::QuitApp);
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

pub fn interact_with_start_tournament_button(
    mut is_popup: ResMut<IsPopup>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<CreateTournamentButton>)
    >,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();

                //set the tournament name
                if is_popup.0 == false {
                    println!("🎯 Create Tournament clicked!");
                    is_popup.0 = true;
                    spawn_request_popup("Entrez le nom de votre tournois:", &mut commands, &asset_server,  windows, PopupPurpose::SetTournamentName);
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