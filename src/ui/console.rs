use super::*;
//use sickle_ui::{ui_builder::{UiBuilderExt, UiRoot}, widgets::layout::container::UiContainerExt};

pub fn draw_console_ui(mut commands: Commands, query: Query<&ActiveConsole>) {
    trace!("draw_console_ui");
    if query.get_single().is_ok() {
        let child = commands
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                overflow: Overflow::clip_y(),
                ..default()
            })
            .id();
        let parent = commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
                ZIndex(20),
                UiConsole,
            ))
            .id();

        commands.queue(AddChild { parent, child });
    }
}
