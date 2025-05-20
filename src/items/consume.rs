use avian_pickup::input::AvianPickupInput;
use bevy::prelude::*;

//use crate::{interact::Interaction, InteractEvent, Interactable};
use crate::{interact::Interaction, InteractEvent};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Consume;

pub struct ComsumePlugin;

impl Interaction for Consume {
    fn interact(
        &self,
        commands: &mut Commands,
        _actor: &Entity,
        prop: &Entity,
//        query: QueryLens<&Interactable>,
    ) -> Option<AvianPickupInput>
    {
        commands.entity(*prop).despawn();
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
    let _actor = trigger.event().actor;
    let _consumeable = trigger.target();
}
