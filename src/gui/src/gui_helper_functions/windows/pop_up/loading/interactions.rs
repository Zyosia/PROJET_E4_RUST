use bevy::prelude::*;
use crate::gui_helper_functions::windows::pop_up::loading::components::{LoadingText, RollAnimation};
use crate::LoadingTimer;

pub fn animate_loading_text_system(
    time: Res<Time>,
    mut loading_timer: ResMut<LoadingTimer>,
    mut query: Query<&mut Text, With<LoadingText>>,
) {
    loading_timer.timer.tick(time.delta());

    if loading_timer.timer.finished() {
        eprintln!("ANIMATION BOI");
        for mut text in &mut query {
            let current = text.0.clone();
            let base = current.trim_end_matches('.').to_string(); // remove all dots

            let dot_count = current.chars().filter(|&c| c == '.').count();
            let next_dot_count = (dot_count + 1) % 4; // cycle 0 -> 1 -> 2 -> 3 -> 0

            text.0 = format!("{}{}", base, ".".repeat(next_dot_count));
        }
    }
}

pub fn roll_circle_system(
    time: Res<Time>,
    mut query: Query<(&RollAnimation, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        // Rotate the circle by 180 degrees per second
        transform.rotation = Quat::from_rotation_z(time.delta().as_secs() as f32 * std::f32::consts::PI);
    }
}
