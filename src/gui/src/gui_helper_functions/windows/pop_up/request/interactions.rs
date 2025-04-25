use bevy::prelude::*;
use crate::gui_helper_functions::windows::pop_up::request::components::*;
use crate::tournament_menu::components::*;

use crate::gui_helper_functions::windows::pop_up::request::styles::*;
use crate::AppState;
use crate::IsPopup;


use bevy_simple_text_input::TextInputValue;

use tournament_engine::Tournament;

pub fn interact_with_cancel_request_pop_up_button(
    mut is_popup: ResMut<IsPopup>,
    mut commands: Commands,
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Entity,
        ),
        (Changed<Interaction>, With<CancelRequestPopupButton>)
    >,
    popup_query: Query<Entity, With<RequestPopup>>,
    overlay_query: Query<Entity, With<RequestPopupOverlay>>,
) {
    if let Ok((interaction, mut color, mut border_color, _button_entity)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = CANCEL_PRESSED_BG.into();
                border_color.0 = CANCEL_PRESSED_BORDER;

                if let Ok(popup_entity) = popup_query.get_single() {
                    if commands.get_entity(popup_entity).is_some() {
                        commands.entity(popup_entity).despawn_recursive();
                    }
                }                                
                if let Ok(overlay_entity) = overlay_query.get_single() {
                    if commands.get_entity(overlay_entity).is_some() {
                        commands.entity(overlay_entity).despawn_recursive();
                    }
                }  

                is_popup.0 = false;
            }
            Interaction::Hovered => {
                *color = CANCEL_HOVERED_BG.into();
                border_color.0 = CANCEL_HOVERED_BORDER;
            }
            Interaction::None => {
                *color = CANCEL_NORMAL_BG.into();
                border_color.0 = CANCEL_NORMAL_BORDER;
            }
        }
    }
}

pub fn interact_with_confirm_request_pop_up_button(
    mut is_popup: ResMut<IsPopup>,
    mut commands: Commands,
    mut tournament: ResMut<Tournament>,
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<ConfirmRequestPopupButton>)
    >,
    input_query: Query<&TextInputValue>,
    popup_query: Query<Entity, With<RequestPopup>>,
    popup_purpose: Query<&PopupPurpose, With<RequestPopup>>,
    overlay_query: Query<Entity, With<RequestPopupOverlay>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut number_text_query: Query<&mut Text, With<NumberOfGamesText>>,
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = CONFIRM_PRESSED_BG.into();
                border_color.0 = CONFIRM_PRESSED_BORDER;

                // Retrieve the input value
                if let Ok(purpose) = popup_purpose.get_single() {
                    if let Ok(input) = input_query.get_single() {
                        match purpose {
                            PopupPurpose::SetTournamentName => {
                                tournament.set_tournament_name(input.0.clone()).unwrap();
                                println!("Tournament name set to: {}", tournament.get_name());
                                tournament.add_all_ai().unwrap();

                                if let Ok(popup_entity) = popup_query.get_single() {
                                    if commands.get_entity(popup_entity).is_some() {
                                        commands.entity(popup_entity).despawn_recursive();
                                    }
                                }                                
                                if let Ok(overlay_entity) = overlay_query.get_single() {
                                    if commands.get_entity(overlay_entity).is_some() {
                                        commands.entity(overlay_entity).despawn_recursive();
                                    }
                                }                                
                                is_popup.0 = false;
                                app_state_next_state.set(AppState::TournamentMenu);
                            }
                            PopupPurpose::SetNumberOfGames => {
                                if let Ok(num) = input.0.parse::<i32>() {
                                    tournament.set_number_of_games(num).unwrap();
                                    println!("Number of games set to: {}", num);

                                    if let Ok(mut text) = number_text_query.get_single_mut() {
                                        text.0 = format!("Parties par match : {}", tournament.get_number_of_games());
                                    }                                    

                                    if let Ok(popup_entity) = popup_query.get_single() {
                                        if commands.get_entity(popup_entity).is_some() {
                                            commands.entity(popup_entity).despawn_recursive();
                                        }
                                    }                                
                                    if let Ok(overlay_entity) = overlay_query.get_single() {
                                        if commands.get_entity(overlay_entity).is_some() {
                                            commands.entity(overlay_entity).despawn_recursive();
                                        }
                                    }
                                    is_popup.0 = false;
                                } else {
                                    println!("Invalid number input");
                                    return;
                                }
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = CONFIRM_HOVERED_BG.into();
                border_color.0 = CONFIRM_HOVERED_BORDER;
            }
            Interaction::None => {
                *color = CONFIRM_NORMAL_BG.into();
                border_color.0 = CONFIRM_NORMAL_BORDER;
            }
        }
    }
}