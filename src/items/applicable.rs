use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::ApplicatorEvent;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_applicable_add)]
pub struct Applicable;

fn on_applicable_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(applicator_event_observer);
}

pub struct ApplicableItemPlugin;
impl Plugin for ApplicableItemPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<Applicable>();
    }
}

fn applicator_event_observer(
    trigger: On<ApplicatorEvent>,
    mut commands: Commands,
    applicable_query: Query<&Applicable>,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    println!("AEIOU");
    if let Ok(applicable) = applicable_query.get(trigger.entity)
    && let Ok(rover) = rover_query.single() {
        println!("QWERTY");
    }
}

