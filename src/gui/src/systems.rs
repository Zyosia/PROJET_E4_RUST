use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2d);
}

pub fn exit_app(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}