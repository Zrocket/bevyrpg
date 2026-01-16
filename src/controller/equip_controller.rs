use crate::{DisplayEquipEvent, OpenEquipAction, Player};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Start;

pub struct EquipControllerPlugin;
impl Plugin for EquipControllerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_observer(open_equip);
    }
}

fn open_equip(
    _trigger: On<Start<OpenEquipAction>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = player_query.single() {
        commands.entity(entity).trigger(|entity| DisplayEquipEvent { entity });
    }
}
