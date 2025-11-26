use bevy::prelude::*;

use crate::widgets;

#[derive(Debug, Component)]
pub struct UiBook;

pub struct BookUiPlugin;
impl Plugin for BookUiPlugin {
    fn build(&self, app: &mut App) {
       app; 
    }
}

fn spawn_book_ui(
    mut commands: Commands,
) {
    commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(50.),
                height: Val::Percent(70.),
                left: Val::Percent(10.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            GlobalZIndex(2),
            UiBook,
    ))
    .with_children(|page_window| {
    });
}
