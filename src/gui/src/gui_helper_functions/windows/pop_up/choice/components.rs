use bevy::prelude::Component;

#[derive(Component)]
pub struct ChoicePopup;

#[derive(Component)]
pub struct YesButton;

#[derive(Component)]
pub struct NoButton;

#[derive(Component)]
pub struct ChoicePopupOverlay;

#[derive(Component)]
pub enum ChoicePopupPurpose {
    QuitApp,
    QuitTournament,
    SaveTournament,
}