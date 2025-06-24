use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::{interact::Interaction, Player};

#[derive(Event)]
pub struct LadderEvent {
    actor: Entity,
    target: Entity,
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct LadderComponent;
impl Interaction for LadderComponent {
    fn interact(&self,commands: &mut Commands,entity:Entity,prop:Entity,) {
        println!("Ladder Interaction");
        commands.trigger_targets(LadderEvent {actor: entity, target: prop}, entity);
    }
}

pub struct LadderPlugin;
impl Plugin for LadderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LadderComponent>()
            .register_component_as::<dyn Interaction, LadderComponent>()
            .add_event::<LadderEvent>()
            .add_observer(ladder_event_observer);
    }
}

fn ladder_event_observer(
    trigger: Trigger<LadderEvent>,
    mut command: Commands,
    player_query: Query<Entity, With<Player>>,
    ladder: Query<Entity, With<LadderComponent>>,
) {
    trace!("OBSERVER: ladder_event_observer");
    if let Ok(ladder_entity) = ladder.single()
        && let Ok(player_entity) = player_query.single() {
    }
}
