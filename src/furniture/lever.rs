use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Interactable, InteractionEvent};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_lever_add)]
pub struct LeverComponent;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct ActivationTargets(Vec<Entity>);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Activatable;

#[derive(Debug, EntityEvent)]
pub struct ActivatableEvent {
    entity: Entity,
}

pub struct LeverPlugin;
impl Plugin for LeverPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LeverComponent>();
    }
}

fn on_lever_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_lever_add");
    world.commands()
        .entity(context.entity)
        .observe(lever_interaction_observer)
        .insert(Interactable);
}

fn lever_interaction_observer(
    _trigger: On<InteractionEvent>,
    lever: Query<Entity, With<LeverComponent>>,
) {
    trace!("OBSERVER: lever_event_observer");
    if let Ok(_lever_entity) = lever.single() {
    }
}
