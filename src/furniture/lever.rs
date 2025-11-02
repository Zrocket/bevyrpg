use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::interact::Interaction;

#[derive(Event)]
pub struct LeverEvent {
    actor: Entity,
    target: Entity,
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct LeverComponent;
impl Interaction for LeverComponent {
    fn interact(&self,commands: &mut Commands,entity:Entity,prop:Entity,) {
        println!("Lever Interaction");
        commands.trigger(LeverEvent {actor: entity, target: prop});
    }
}

pub struct LeverPlugin;
impl Plugin for LeverPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LeverComponent>()
            .register_component_as::<dyn Interaction, LeverComponent>()
            //.add_event::<LeverEvent>()
            .add_observer(lever_event_observer);
    }
}

fn lever_event_observer(
    _trigger: On<LeverEvent>,
    lever: Query<Entity, With<LeverComponent>>,
) {
    trace!("OBSERVER: lever_event_observer");
    if let Ok(_lever_entity) = lever.single() {
    }
}
