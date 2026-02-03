use bevy::prelude::*;
use bevy_enhanced_input::prelude::Start;

use crate::{DisplayQuestEvent, OpenQuestAction, Player};

pub struct QuestControllerPlugin;
impl Plugin for QuestControllerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_observer(open_quest);
    }
}

fn open_quest(
    _trigger: On<Start<OpenQuestAction>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = player_query.single() {
        commands.entity(entity).trigger(|entity| DisplayQuestEvent { entity });
    }
}
