use bevy::prelude::*;
use crate::gui_helper_functions::windows::pop_up::information::components::*;
use crate::gui_helper_functions::windows::pop_up::information::styles::*;
use crate::IsPopup;

pub fn interact_with_ok_information_pop_up_button(
    mut is_popup: ResMut<IsPopup>,
    mut commands: Commands,
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Entity,
        ),
        (Changed<Interaction>, With<OkInformationButton>)
    >,
    popup_query: Query<Entity, With<InformationPopup>>,
    overlay_query: Query<Entity, With<InformationPopupOverlay>>,
) {
    if let Ok((interaction, mut color, mut border_color, _button_entity)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = OK_PRESSED_BG.into();
                border_color.0 = OK_PRESSED_BORDER;

                if let Ok(popup_entity) = popup_query.get_single() {
                    commands.entity(popup_entity).despawn_recursive();
                }
                if let Ok(overlay_entity) = overlay_query.get_single() {
                    commands.entity(overlay_entity).despawn_recursive();
                }

                is_popup.0 = false;
            }
            Interaction::Hovered => {
                *color = OK_HOVERED_BG.into();
                border_color.0 = OK_HOVERED_BORDER;
            }
            Interaction::None => {
                *color = OK_NORMAL_BG.into();
                border_color.0 = OK_NORMAL_BORDER;
            }
        }
    }
}