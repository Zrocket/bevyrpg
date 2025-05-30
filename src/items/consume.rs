use bevy::prelude::*;

use crate::{interact::Interaction, InteractEvent};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Consume;

pub struct ComsumePlugin;

impl Interaction for Consume {
    fn interact(
        &self,
        commands: &mut Commands,
        _actor: Entity,
        prop: Entity,
    ) {
        commands.entity(prop).despawn();
    }
}

impl Plugin for ComsumePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Consume>()
            .add_observer(consume_observer_handler);
    }
}

fn consume_observer_handler(
    trigger: Trigger<InteractEvent, Consume>
) {
    let _actor = trigger.event().actor;
    let _consumeable = trigger.target();
}
