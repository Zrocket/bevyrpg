use bevy::prelude::*;
use bevy_fps_controller::controller::*;
use crate::{ActiveInventoryUi, ActiveUi, Player, UiIndex, UiInventory};

pub fn manage_inventory(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut fps_controller: Query<&mut FpsController>,
    mut player: Query<(Entity, Option<&ActiveInventoryUi>), With<Player>>,
) {
    if key.just_pressed(KeyCode::KeyI) {
        info!("KeyI pressed");
        if let Ok(mut fps_controller) =fps_controller.get_single_mut() {
            info!("Got fps_controller");
                if let Ok((player, active)) = player.get_single_mut() {
                    if active.is_none() {
                        info!("adding ActiveUi");
                        commands.entity(player).insert(ActiveUi);
                        commands.entity(player).insert(ActiveInventoryUi);
                    } else {
                        info!("removing ActiveUi");
                        commands.entity(player).remove::<ActiveUi>();
                        commands.entity(player).remove::<ActiveInventoryUi>();
                    }
                }
                fps_controller.enable_input = !fps_controller.enable_input;
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
