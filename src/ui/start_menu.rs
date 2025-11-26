use bevy::prelude::*;

use crate::widgets;

#[derive(Component, Reflect)]
pub struct UiStartMenu;

pub struct StartMenuUiPlugin;
impl Plugin for StartMenuUiPlugin {
    fn build(&self, app: &mut App) {
       app; 
    }
}

fn draw_start_menu(
    mut commands: Commands,
) {
    commands.spawn((
        widgets::ui_root("Start Menu"),
        GlobalZIndex(2),
        children![
        ]
    ));
}
