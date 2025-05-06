use avian_pickup::input::AvianPickupInput;
use bevy::{ecs::system::QueryLens, prelude::*};

use crate::{interact::Interaction, InteractEvent, Interactable};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Consume;

pub struct ComsumePlugin;

impl Interaction for Consume {
    fn interact(
        &self,
        actor: &Entity,
        query: QueryLens<&Interactable>
    ) -> Option<AvianPickupInput>
    {
        None
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
    let actor = trigger.event().actor;
    let consumeable = trigger.entity();
}
