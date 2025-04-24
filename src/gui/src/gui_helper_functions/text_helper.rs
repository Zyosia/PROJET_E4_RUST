use bevy::prelude::*;

pub fn spawn_centered_info_text(parent: &mut ChildBuilder, text: &str, font: Handle<Font>, font_size: f32, color: Color) {
    parent.spawn(Node {
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    })
    .with_children(|node| {
        node.spawn((
            Text::new(text.to_string()),
            TextFont {
                font,
                font_size,
                ..default()
            },
            TextColor(color),
        ));
    });
}
