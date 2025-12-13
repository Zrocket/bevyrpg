use bevy::prelude::*;

use crate::{Book, OpenBookEvent, widgets};

#[derive(Debug, Component)]
pub struct UiBook;

#[derive(EntityEvent)]
pub struct SpawnBookUiEvent {
    entity: Entity,
}

pub struct BookUiPlugin;
impl Plugin for BookUiPlugin {
    fn build(&self, app: &mut App) {
       app; 
    }
}

fn spawn_book_ui(
    trigger: On<OpenBookEvent>,
    mut commands: Commands,
    book_query: Query<&Book>,
) {
    if let Ok(book) = book_query.get(trigger.observer()) {
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
            page_window.spawn((
                    Text(book.title.to_string()),
                    Text(book.contents.to_string()),
            ));
        });
    }
}
