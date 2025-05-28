use avian_pickup::{input::{AvianPickupAction, AvianPickupInput}, prop::HeldProp};
use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::{interact::Interaction, InteractEvent};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MiscItem;

#[derive(Event)]
pub struct MiscInteractEvent {
    actor: Entity,
    prop: Entity,
}

pub struct MiscItemPlugin;

impl Interaction for MiscItem {
    fn interact(
        &self,
        commands: &mut Commands,
        actor: Entity,
        prop: Entity,
    ) {
        println!("Misc Interaction Impl");
        commands.trigger_targets(MiscInteractEvent {actor, prop}, prop);
    }
}

impl Plugin for MiscItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MiscItem>()
            .register_component_as::<dyn Interaction, MiscItem>()
            .add_event::<MiscInteractEvent>()
            .add_observer(misc_observer_handler);
    }
}

fn misc_observer_handler(
    trigger: Trigger<MiscInteractEvent>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
    _held_prop_query: Query<&HeldProp>,
) {
    info!("Misc Interact event");
    let actor = trigger.event().actor;
    avian_pickup_input_writer.write(AvianPickupInput { actor, action: AvianPickupAction::Pull });
}
