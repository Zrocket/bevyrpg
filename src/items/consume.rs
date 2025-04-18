use bevy::prelude::*;

use crate::InteractEvent;

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Consume;

pub struct ComsumePlugin;

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
