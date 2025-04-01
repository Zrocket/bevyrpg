use crate::ActiveUi;
use bevy::prelude::*;

use super::GameState;

#[derive(Event)]
pub struct TradeEvent {
    pub actor: Entity,
    pub target: Entity,
}

pub struct TradePlugin;

impl Plugin for TradePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TradeEvent>().add_systems(
            Update,
            trade_event_handler.run_if(in_state(GameState::Gameplay)),
        );
    }
}

fn trade_event_handler(
    mut command: Commands,
    mut trade_query: Query<Entity>,
    mut trade_events: EventReader<TradeEvent>,
) {
    for event in trade_events.read() {
        if let Ok(_actor) = trade_query.get_mut(event.actor) {
            if let Ok(target) = trade_query.get_mut(event.target) {
                command.entity(target).insert(ActiveUi);
            }
        }
    }
}
