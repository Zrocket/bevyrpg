use avian_pickup::{input::{AvianPickupAction, AvianPickupInput}, prop::HeldProp};
use bevy::{ecs::system::QueryLens, prelude::*};

use crate::{interact::Interaction, InteractEvent, Interactable};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MiscItem;

pub struct MiscItemPlugin;

impl Interaction for MiscItem {
    fn interact(
        &self,
        actor: &Entity,
        mut interactable_query: QueryLens<&Interactable>,
    ) -> Option<AvianPickupInput>
    {
        println!("AAAAAAAAAAAAAAAAAa");
        interactable_query.query().get(*actor);
        //interactable_query.get(*actor);
        Some(AvianPickupInput { actor: *actor, action: AvianPickupAction::Pull })
    }
}

impl Plugin for MiscItemPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_type::<MiscItem>()
            .register_component_as::<dyn Interaction, MiscItem>()
            .add_observer(misc_observer_handler);
    }
}

fn misc_observer_handler(
    trigger: Trigger<InteractEvent, (MiscItem)>,
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
