use crate::{GameState, Player, UiIndex, UiInventory};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use super::Action;

pub struct InventoryControllerPlugin;
impl Plugin for InventoryControllerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(Update, open_inventory.run_if(in_state(GameState::Gameplay)))
           .add_systems(Update, close_inventory.run_if(in_state(GameState::Inventory)));
    }
}

fn open_inventory(
    key: Query<&ActionState<Action>, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(key) = key.single() && key.just_pressed(&Action::OpenInventory) {
        game_state.set(GameState::Inventory);
    }
}

fn close_inventory(
    key: Query<&ActionState<Action>, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(key) = key.single() && key.just_pressed(&Action::OpenInventory) {
        game_state.set(GameState::Gameplay);
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
