use bevy::prelude::*;

use crate::widgets;

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
            widgets::ui_root("Book UI"),
            GlobalZIndex(2),
    ));
}
