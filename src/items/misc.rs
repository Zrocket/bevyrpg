use avian_pickup::{input::{AvianPickupAction, AvianPickupInput}, prop::HeldProp};
use bevy::prelude::*;

use crate::{interact::Interaction, InteractEvent};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MiscItem;

pub struct MiscItemPlugin;

impl Interaction for MiscItem {
    fn interact(
        &self,
        _commands: &mut Commands,
        actor: &Entity,
        _prop: &Entity,
    ) -> Option<AvianPickupInput>
    {
        println!("Misc Interaction Impl");
        Some(AvianPickupInput { actor: *actor, action: AvianPickupAction::Pull })
    }
}

impl Plugin for MiscItemPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_type::<MiscItem>()
            .register_component_as::<dyn Interaction, MiscItem>();
            //.add_observer(misc_observer_handler);
    }
}

fn _misc_observer_handler(
    trigger: Trigger<InteractEvent, MiscItem>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
    _held_prop_query: Query<&HeldProp>,
) {
    info!("Misc Interact event");
    let actor = trigger.event().actor;
    avian_pickup_input_writer.send(AvianPickupInput { actor, action: AvianPickupAction::Pull });
}
