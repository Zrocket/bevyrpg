use crate::{DisplayInventoryEvent, OpenInventoryAction, Player, UiIndex, UiInventory};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::{Start};

pub struct InventoryControllerPlugin;
impl Plugin for InventoryControllerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_observer(open_inventory);
    }
}

fn open_inventory(
    _trigger: On<Start<OpenInventoryAction>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = player_query.single() {
        commands.entity(entity).trigger(|entity| DisplayInventoryEvent { entity });
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
