use bevy::prelude::*;
use bevy::ecs::system::ParamSet;
use bevy::window::PrimaryWindow;
use bevy_simple_text_input::TextInputValue;

use crate::gui_helper_functions::windows::pop_up::loading::loading::spawn_loading_popup;
use crate::gui_helper_functions::windows::pop_up::information::information::spawn_information_popup;
use crate::gui_helper_functions::windows::pop_up::request::request::{spawn_request_popup, despawn_request_popup};

use crate::gui_helper_functions::windows::pop_up::request::components::*;
use crate::gui_helper_functions::windows::pop_up::loading::components::*;
use crate::tournament_menu::components::*;
use crate::ai_insert_menu::components::*;

use crate::gui_helper_functions::windows::pop_up::request::styles::*;
use crate::PendingTournamentName;
use crate::AppState;
use crate::IsPopup;

use tournament_engine::tournament::Tournament;
use ai_manager::{ai_creator::*, helper_functions::ai_exists};

use rfd::FileDialog;

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

                despawn_request_popup(&mut commands, popup_query, overlay_query);  

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
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
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
    popup_purpose: Query<&PopupPurpose, With<RequestPopup>>,
    mut popups_query: ParamSet<(
        Query<Entity, With<RequestPopup>>,
        Query<Entity, With<LoadingPopup>>,
    )>,
    mut overlays_query: ParamSet<(
        Query<Entity, With<RequestPopupOverlay>>,
        Query<Entity, With<LoadingPopupOverlay>>,
    )>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<NumberOfGamesText>>,
        Query<&mut Text, With<PathAIText>>,
    )>,
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
                                despawn_request_popup(&mut commands, popups_query.p0(), overlays_query.p0());
                                spawn_loading_popup(
                                    "Création du tournoi en cours . . . ", 
                                    &mut commands, 
                                    &asset_server, 
                                    windows
                                );
                                commands.insert_resource(PendingTournamentName(input.0.clone()));
                                app_state_next_state.set(AppState::CreatingTournament);
                            }
                            PopupPurpose::SetNumberOfGames => {
                                if let Ok(num) = input.0.parse::<i32>() {
                                    tournament.set_number_of_games(num).unwrap();
                                    println!("Number of games set to: {}", num);

                                    if let Ok(mut text) = text_queries.p0().get_single_mut() {
                                        text.0 = format!("Parties par match : {}", tournament.get_number_of_games());
                                    }                                    

                                    despawn_request_popup(&mut commands, popups_query.p0(), overlays_query.p0());  

                                    is_popup.0 = false;
                                } else {
                                    println!("Invalid number input");
                                    return;
                                }
                            }
                            PopupPurpose::SetAiName => {

                                despawn_request_popup(&mut commands, popups_query.p0(), overlays_query.p0());

                                if ai_exists(&input.0.clone()) {
                                    spawn_request_popup("Ce nom d'IA est déjà présent!\nVeuillez en saisir un autre:", &mut commands, &asset_server, windows, PopupPurpose::SetAiName);
                                } else {                                    
                                    if let Some(path) = FileDialog::new().pick_file() {
                                        println!("Fichier sélectionné : {:?}", path);
                                        
                                        match create_ai_from_file(&input.0.clone(), &path) {
                                            Ok(()) => println!("✅ AI package créé avec succès!"),
                                            Err(e) => eprintln!("❌ Echec création du package: {}", e),
                                        }
                                        
                                        match tournament.add_all_ai(){
                                            Ok(()) => println!("✅ IA ajouté au tournois avec succès"),
                                            Err(e) => eprintln!("❌ Echec ajout de l'IA au tournois {}", e),
                                        }

                                        //Si succès
                                        if let Ok(mut text) = text_queries.p1().get_single_mut() {
                                            text.0 = format!("IA {} ajouté avec succès !", input.0.clone());
                                        }

                                    } else {
                                        spawn_information_popup("Aucun Fichier sélectionné\nAnnulation ajout de l'IA.", &mut commands, &asset_server, windows);
                                        return;
                                    }
                                    is_popup.0 = false;
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