use bevy::{prelude::*, window::CursorGrabMode};
use leafwing_input_manager::prelude::ActionState;
use crate::{ActiveInventoryUi, ActiveUi, Player, UiIndex, UiInventory};

use super::Action;

pub fn manage_inventory(
    mut commands: Commands,
    key: Query<&ActionState<Action>, With<Player>>,
    mut window: Query<&mut Window>,
    mut player: Query<(Entity, Option<&ActiveInventoryUi>), With<Player>>,
) {
    if let Ok(key) = key.get_single() {
        if key.just_pressed(&Action::OpenInventory) {
            info!("KeyI pressed");
            if let Ok((player, active)) = player.get_single_mut() {
                if active.is_none() {
                    info!("adding ActiveUi");
                    commands.entity(player).insert(ActiveUi);
                    commands.entity(player).insert(ActiveInventoryUi);
                    if let Ok(mut window) = window.get_single_mut() {
                        window.cursor.grab_mode = CursorGrabMode::None;
                        window.cursor.visible = true;
                    }
                } else {
                    info!("removing ActiveUi");
                    commands.entity(player).remove::<ActiveUi>();
                    commands.entity(player).remove::<ActiveInventoryUi>();
                    if let Ok(mut window) = window.get_single_mut() {
                        window.cursor.grab_mode = CursorGrabMode::Locked;
                        window.cursor.visible = false;
                    }
                }
            }
        }
    }
}

pub fn inventory_navigation(
    key: Res<ButtonInput<KeyCode>>,
    mut index_query: Query<&mut UiIndex, With<UiInventory>>,
                           ) {
    for mut index in index_query.iter_mut(){
        // navigate up
        if (key.just_pressed(KeyCode::KeyW) || key.just_pressed(KeyCode::ArrowUp)) && index.0 > 0 {
            index.0 -= 1;
        // navigate down
        } else if key.just_pressed(KeyCode::KeyS) || key.just_pressed(KeyCode::ArrowDown) {
            index.0 += 1;
        // select
        } else if key.just_pressed(KeyCode::Enter) {
            todo!();
        }
    }
}
