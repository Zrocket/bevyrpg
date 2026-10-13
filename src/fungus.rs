use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{ApplicatorAttachment, ApplicatorEvent, ApplicatorSubstance, ItemId};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    crate::Drillable("Fungus".into()),
)]
#[component(on_add = on_fungus_add)]
pub struct Fungus;

fn on_fungus_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(fungus_applicator_event_observer);
}

pub struct FungusPlugin;
impl Plugin for FungusPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<Fungus>();
    }
}

fn fungus_applicator_event_observer(
    trigger: On<ApplicatorEvent>,
    mut commands: Commands,
    //applicable_query: Query<&crate::Applicable>,
    rover_query: Query<Entity, With<crate::Rover>>,
    applicator_attachment_query: Query<&ApplicatorAttachment>,
    substance_query: Query<(&ApplicatorSubstance, &ItemId)>,
) {
    trace!("OBSERVER: funngus_applicator_event_observer");
    //let Ok(applicable) = applicable_query.get(trigger.entity) else {
    //    error!("fungus_applicator_event_observer: Failed to query Applicable for {}", trigger.entity);
    //    return;
    //};
    let Ok(rover) = rover_query.single() else {
        error!("fungus_applicator_event_observer: Failed to query Rover");
        return;
    };
    let Ok(applicator_attachment) = applicator_attachment_query.single() else {
        error!("fungus_applicator_event_observer: Failed to query ApplicatorAttachment");
        return;
    };
    if applicator_attachment.0.is_some() {
        let Ok((substance, item_id)) = substance_query.get(applicator_attachment.0.unwrap()) else {
            error!("fungus_applicator_event_observer: Failed to query ApplicatorSubstance and ItemId");
            return;
        };
        if item_id.0 == "colloidal_copper" {
            println!("SSSSSSSSSSSSSSSSSSSSSSSSSS");
        }
    }
}
