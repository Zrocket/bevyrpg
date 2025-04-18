use avian_pickup::{input::{AvianPickupAction, AvianPickupInput}, prop::HeldProp};
use bevy::prelude::*;

use crate::InteractEvent;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MiscItem;

pub struct MiscItemPlugin;

impl Plugin for MiscItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MiscItem>()
            .add_observer(misc_observer_handler);
    }
}

fn misc_observer_handler(
    trigger: Trigger<InteractEvent, MiscItem>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
    held_prop_query: Query<&HeldProp>,
) {
    info!("Misc Interact event");
    let actor = trigger.event().actor;

    if let Ok(_held_prop) = held_prop_query.get_single() {
        avian_pickup_input_writer.send( AvianPickupInput { actor, action: AvianPickupAction::Drop } );
    } else {
        avian_pickup_input_writer.send(AvianPickupInput { actor, action: AvianPickupAction::Pull });
    }
}
