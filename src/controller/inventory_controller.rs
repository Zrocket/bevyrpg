use crate::{DisplayInventoryEvent, GameState, OpenInventoryAction, Player, UiIndex, UiInventory};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::{Fire, Start};
use leafwing_input_manager::prelude::ActionState;

//use super::LeafwingAction;

pub struct InventoryControllerPlugin;
impl Plugin for InventoryControllerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_observer(bei_open_inventory);
           //.add_systems(Update, open_inventory.run_if(in_state(GameState::Gameplay)));
           //.add_systems(Update, close_inventory.run_if(in_state(GameState::Inventory)));
    }
}

fn bei_open_inventory(
    trigger: On<Start<OpenInventoryAction>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = player_query.single() {
        commands.entity(entity).trigger(|entity| DisplayInventoryEvent { entity });
    }
}

/*fn open_inventory(
    key: Query<&ActionState<LeafwingAction>, With<Player>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = player_query.single()
    && let Ok(key) = key.single() && key.just_pressed(&LeafwingAction::OpenInventory) {
        commands.entity(entity).trigger(|entity| DisplayInventoryEvent { entity });
    }
}*/

/*fn close_inventory(
    key: Query<&ActionState<Action>, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(key) = key.single() && key.just_pressed(&Action::OpenInventory) {
        game_state.set(GameState::Gameplay);
    }
}*/

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
