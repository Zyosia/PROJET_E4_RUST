use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::gui_helper_functions::windows::pop_up::choice::components::*;
use crate::gui_helper_functions::windows::pop_up::choice::choice::*;

use crate::gui_helper_functions::windows::pop_up::choice::styles::*;
use crate::AppState;
use crate::IsPopup;
use crate::ResetTournamentEvent;

use tournament_engine::{tournament::{Tournament, TournamentState}, helper::save_tournament};
use game_history::helper::delete_all_history_files;

pub fn interact_with_no_pop_up_button(
    mut is_popup: ResMut<IsPopup>,
    mut commands: Commands,
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Entity,
        ),
        (Changed<Interaction>, With<NoButton>)
    >,
    popup_purpose: Query<&ChoicePopupPurpose, With<ChoicePopup>>,
    popup_query: Query<Entity, With<ChoicePopup>>,
    overlay_query: Query<Entity, With<ChoicePopupOverlay>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut reset_writer: EventWriter<ResetTournamentEvent>,
) {
    if let Ok((interaction, mut color, mut border_color, _button_entity)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = CANCEL_PRESSED_BG.into();
                border_color.0 = CANCEL_PRESSED_BORDER;

                if let Ok(purpose) = popup_purpose.get_single() {
                    match purpose {
                        ChoicePopupPurpose::QuitApp => {
                            despawn_choice_popup(&mut commands, popup_query, overlay_query);           
                            is_popup.0 = false;
                        } // PopupPurpose::QuitApp
                        ChoicePopupPurpose::QuitTournament => {
                            despawn_choice_popup(&mut commands, popup_query, overlay_query);
                            is_popup.0 = false;

                        } // PopupPurpose::QuitTournament
                        ChoicePopupPurpose::SaveTournament => {
                                // Delete all history files
                                if let Err(e) = delete_all_history_files() {
                                    println!("❌ Erreur suppression de l'historique: {}", e);
                                }
                                despawn_choice_popup(&mut commands, popup_query, overlay_query);
                                // ✔ Schedule the state change
                                app_state_next_state.set(AppState::MainMenu);
                                // ✔ Send the event
                                reset_writer.send(ResetTournamentEvent);
                                is_popup.0 = false;
                        } // PopupPurpose::SaveTournament
                    }
                }
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

pub fn interact_with_yes_pop_up_button(
    mut is_popup: ResMut<IsPopup>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    tournament: Res<Tournament>,
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<YesButton>)
    >,
    popup_query: Query<Entity, With<ChoicePopup>>,
    popup_purpose: Query<&ChoicePopupPurpose, With<ChoicePopup>>,
    overlay_query: Query<Entity, With<ChoicePopupOverlay>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut reset_writer: EventWriter<ResetTournamentEvent>,
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = CONFIRM_PRESSED_BG.into();
                border_color.0 = CONFIRM_PRESSED_BORDER;

                // Retrieve the input value
                if let Ok(purpose) = popup_purpose.get_single() {
                    match purpose {
                        ChoicePopupPurpose::QuitApp => {
                            despawn_choice_popup(&mut commands, popup_query, overlay_query); 
                            app_exit_event_writer.send(AppExit::Success);              
                            is_popup.0 = false;
                        } // PopupPurpose::QuitApp
                        ChoicePopupPurpose::QuitTournament => {
                            despawn_choice_popup(&mut commands, popup_query, overlay_query);
                            if tournament.get_state() == TournamentState::Started {
                                spawn_choice_popup("Enregistrer le tournois ?", &mut commands, &asset_server, windows, ChoicePopupPurpose::SaveTournament)
                            } else {
                                is_popup.0 = false;
                                // ✔ Schedule the state change
                                app_state_next_state.set(AppState::MainMenu);
                                // ✔ Send the event
                                reset_writer.send(ResetTournamentEvent);
                            }

                        } // PopupPurpose::QuitTournament
                        ChoicePopupPurpose::SaveTournament => {
                            //Save the tournament 
                            if let Err(e) = save_tournament(&tournament){
                                println!("❌ Erreur sauvegarde du tournoi: {}", e);
                            }
                            // Delete all history files
                            if let Err(e) = delete_all_history_files() {
                                println!("❌ Erreur suppression de l'historique: {}", e);
                            }

                            despawn_choice_popup(&mut commands, popup_query, overlay_query);
                            is_popup.0 = false;
                            // ✔ Schedule the state change
                            app_state_next_state.set(AppState::MainMenu);
                            // ✔ Send the event
                            reset_writer.send(ResetTournamentEvent);
                        } // PopupPurpose::SaveTournament
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