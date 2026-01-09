use crate::{DisplayEquipEvent, DisplayStatsEvent, GameState, Player, UiIndex, UiInventory};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use super::Action;

pub struct StatsControllerPlugin;
impl Plugin for StatsControllerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(Update, open_stats.run_if(in_state(GameState::Gameplay)));
           //.add_systems(Update, close_equip.run_if(in_state(GameState::Inventory)));
    }
}

fn open_stats(
    key: Query<&ActionState<Action>, With<Player>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = player_query.single()
    && let Ok(key) = key.single() && key.just_pressed(&Action::OpenStats) {
        commands.entity(entity).trigger(|entity| DisplayStatsEvent { entity });
    }
}

fn close_stats(
    key: Query<&ActionState<Action>, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(key) = key.single() && key.just_pressed(&Action::OpenInventory) {
        game_state.set(GameState::Gameplay);
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
