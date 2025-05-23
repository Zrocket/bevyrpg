use crate::{Player, UiIndex, UiInventory};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use super::Action;

pub fn manage_inventory(
    //mut commands: Commands,
    key: Query<&ActionState<Action>, With<Player>>,
    //mut window: Query<&mut Window>,
    //mut player: Query<(Entity, Option<&ActiveInventoryUi>), With<Player>>,
    mut inventory_node_query: Query<&mut Node, With<UiInventory>>
) {
    if let Ok(key) = key.single() {
        if key.just_pressed(&Action::OpenInventory) {
            info!("Inventory key pressed");
            if let Ok(mut inventory_node) = inventory_node_query.single_mut() {
            info!("AAAAAAAAAAAAAa");
                match inventory_node.display {
                    Display::None => inventory_node.display = Display::Flex,
                    _ => inventory_node.display = Display::None,
                }
            }
        }
    }
}

pub fn inventory_navigation(
    key: Res<ButtonInput<KeyCode>>,
    mut index_query: Query<&mut UiIndex, With<UiInventory>>,
) {
    for mut index in index_query.iter_mut() {
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
