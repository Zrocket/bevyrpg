use avian_pickup::{input::{AvianPickupAction, AvianPickupInput}, prop::HeldProp};
use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::{interact::Interaction, Inspectable};

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
        info!("Misc Interaction Impl");
        commands.trigger(MiscInteractEvent {actor, prop});
    }
}

impl Inspectable for MiscItem {
    fn inspect(
        &self,
        commands: &mut Commands,
        actor: Entity,
        prop: Entity,
    ) {
        println!("Misc Inspectable Impl");
    }
}

impl Plugin for MiscItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MiscItem>()
            .register_component_as::<dyn Interaction, MiscItem>()
            .register_component_as::<dyn Inspectable, MiscItem>()
            //.add_event::<MiscInteractEvent>()
            .add_observer(misc_event_observer);
    }
}

fn misc_event_observer(
    trigger: On<MiscInteractEvent>,
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
    _held_prop_query: Query<&HeldProp>,
) {
    info!("Misc Interact event observer");
    let actor = trigger.event().actor;
    avian_pickup_input_writer.write(AvianPickupInput { actor, action: AvianPickupAction::Pull });
}
