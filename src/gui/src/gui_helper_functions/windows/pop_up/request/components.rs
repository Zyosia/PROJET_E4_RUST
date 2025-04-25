use bevy::prelude::Component;

#[derive(Component)]
pub struct RequestPopup;

#[derive(Component)]
pub struct ConfirmRequestPopupButton;

#[derive(Component)]
pub struct CancelRequestPopupButton;

#[derive(Component)]
pub struct RequestPopupOverlay;

#[derive(Component)]
pub enum PopupPurpose {
    SetTournamentName,
    SetNumberOfGames,
    SetAiName,
}