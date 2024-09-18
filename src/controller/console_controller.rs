use bevy::prelude::*;
use bevy_fps_controller::controller::*;
use crate::ActiveConsole;

pub fn manage_console(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut fps_controller: Query<&mut FpsController>,
    query: Query<Entity, With<ActiveConsole>>,
) {
    if key.just_pressed(KeyCode::Backslash) {
        if let Ok(mut fps_controller) = fps_controller.get_single_mut() {
            fps_controller.enable_input = !fps_controller.enable_input;
            if let Ok(console_flag) = query.get_single() {
                commands.entity(console_flag).despawn_recursive();
            } else {
                commands.spawn(ActiveConsole);
            }
        }
    }
}
