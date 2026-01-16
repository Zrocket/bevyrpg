use crate::{DisplayStatsEvent, OpenStatsAction, Player, UiIndex, UiInventory};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Start;

pub struct StatsControllerPlugin;
impl Plugin for StatsControllerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_observer(open_stats);
    }
}

fn open_stats(
    _trigger: On<Start<OpenStatsAction>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = player_query.single() {
        commands.entity(entity).trigger(|entity| DisplayStatsEvent { entity });
    }

}

pub fn stats_navigation(
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
