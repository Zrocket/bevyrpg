use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Interactable, InteractionEvent};

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default)]
pub enum DoorState {
    #[default]
    Closed,
    Open,
}

#[derive(Resource)]
pub struct DoorAnimation(pub Handle<AnimationClip>);
#[derive(Resource)]
pub struct DoorGraph(pub Handle<AnimationGraph>);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_door_add)]
pub struct DoorComponent;

pub struct DoorPlugin;
impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DoorComponent>();
    }
}

fn on_door_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_door_add");
    world.commands()
        .entity(context.entity)
        .observe(door_interaction_observer)
        .insert(Interactable);
}

fn door_interaction_observer(
    _trigger: On<InteractionEvent>,
    mut door: Query<(Entity, &mut AnimationPlayer)>,
) {
    trace!("OBSERVER: door_event_observer");
    if let Ok((_door_entity, mut door_animation_player)) = door.single_mut() {
        door_animation_player.play(1.into());
    }
}
