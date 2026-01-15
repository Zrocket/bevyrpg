use crate::{DisplayEquipEvent, DisplayInventoryEvent, GameState, OpenEquipAction, Player, UiIndex, UiInventory};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Start;
use leafwing_input_manager::prelude::ActionState;

pub struct EquipControllerPlugin;
impl Plugin for EquipControllerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_observer(bei_open_equip);
           //.add_systems(Update, open_equip.run_if(in_state(GameState::Gameplay)));
           //.add_systems(Update, close_equip.run_if(in_state(GameState::Inventory)));
    }
}

fn bei_open_equip(
    trigger: On<Start<OpenEquipAction>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = player_query.single() {
        commands.entity(entity).trigger(|entity| DisplayEquipEvent { entity });
    }
}

/*fn open_equip(
    key: Query<&ActionState<LeafwingAction>, With<Player>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = player_query.single()
    && let Ok(key) = key.single() && key.just_pressed(&LeafwingAction::OpenInventory) {
        commands.entity(entity).trigger(|entity| DisplayEquipEvent { entity });
    }
}*/

/*fn close_equip(
    key: Query<&ActionState<LeafwingAction>, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(key) = key.single() && key.just_pressed(&LeafwingAction::OpenInventory) {
        game_state.set(GameState::Gameplay);
    }
}*/

pub fn equip_navigation(
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
