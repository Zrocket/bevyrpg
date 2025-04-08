use bevy_simple_text_input::{TextInput, TextInputInactive, TextInputTextColor, TextInputTextFont};

use super::*;

pub fn draw_console_ui(mut commands: Commands) {
    trace!("draw_console_ui");
    let child = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                overflow: Overflow::clip_y(),
                ..default()
            },
            TextInput,
            TextInputTextFont(TextFont {
                font_size: 34.,
                ..default()
            }),
            TextInputTextColor(TextColor(Color::srgb(0.9, 0.9, 0.9))),
        ))
        .id();
    let parent = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                width: Val::Percent(100.),
                height: Val::Percent(50.),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            ZIndex(20),
            UiConsole,
            TextInputInactive(true),
        ))
        .id();

    commands.queue(AddChild { parent, child });
}
