use bevy::prelude::*;
use crate::gui_helper_functions::windows::pop_up::request::components::*;
use crate::gui_helper_functions::windows::pop_up::request::styles::*;
use crate::IsPopup;

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
) {
    if let Ok((interaction, mut color, mut border_color, _button_entity)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = CANCEL_PRESSED_BG.into();
                border_color.0 = CANCEL_PRESSED_BORDER;

                if let Ok(popup_entity) = popup_query.get_single() {
                    commands.entity(popup_entity).despawn_recursive();
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
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<ConfirmRequestPopupButton>)
    >,
) {
    if let Ok((interaction, mut color, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = CONFIRM_PRESSED_BG.into();
                border_color.0 = CONFIRM_PRESSED_BORDER;
                is_popup.0 = false;
                // Insert your confirm logic here
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