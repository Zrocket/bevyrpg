use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use crate::{ActiveConsole, Player};

use super::Action;

pub fn manage_console(
    mut commands: Commands,
    key: Query<&ActionState<Action>, With<Player>>,
    query: Query<Entity, With<ActiveConsole>>,
) {
    if let Ok(key) = key.get_single() {
        if key.just_pressed(&Action::OpenConsole) {
            info!("Backslash pressed");
            if let Ok(console_flag) = query.get_single() {
                commands.entity(console_flag).despawn_recursive();
            } else {
                commands.spawn(ActiveConsole);
            }
        }
    }
}
