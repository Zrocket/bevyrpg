use bevy::prelude::*;
use bevy_fps_controller::controller::*;
use crate::{ActiveMenuUi, ActiveUi, Player, UiIndex, UiMenu};

pub fn manage_menu(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut fps_controller: Query<&mut FpsController>,
    mut player: Query<(Entity, Option<&ActiveUi>), With<Player>>,
) {
    if key.just_pressed(KeyCode::Semicolon) {
        info!("ESC pressed");
        if let Ok(mut fps_controller) =fps_controller.get_single_mut() {
            info!("Got fps_controller");
                if let Ok((player, active)) = player.get_single_mut() {
                    if active.is_none() {
                        info!("adding ActiveUi");
                        info!("adding ActiveMenuUi");
                        commands.entity(player).insert(ActiveUi);
                        commands.entity(player).insert(ActiveMenuUi);
                    } else {
                        info!("removing ActiveUi");
                        info!("removing ActiveMenuUi");
                        commands.entity(player).remove::<ActiveUi>();
                        commands.entity(player).remove::<ActiveMenuUi>();
                    }
                }
                fps_controller.enable_input = !fps_controller.enable_input;
        }
    }
}

pub fn menu_navigation(
    key: Res<ButtonInput<KeyCode>>,
    mut index_query: Query<&mut UiIndex, With<UiMenu>>,
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
