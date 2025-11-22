use bevy::prelude::*;

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
pub struct DoorComponent;

pub struct DoorPlugin;
impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DoorComponent>()
            .add_systems(Update, register_door_interactions);
    }
}

fn register_door_interactions(
    mut commands: Commands,
    mut doors_query: Query<Entity, (With<DoorComponent>, Without<Interactable>)>,
) {
    for door in doors_query.iter_mut() {
        commands.entity(door).observe(door_interaction_observer)
            .insert(Interactable);
    }
}

fn door_interaction_observer(
    trigger: On<InteractionEvent>,
    mut door: Query<(Entity, &mut AnimationPlayer)>,
) {
    println!("CCCCCCCCCCC");
    trace!("OBSERVER: door_event_observer");
    if let Ok((_door_entity, mut door_animation_player)) = door.single_mut() {
        door_animation_player.play(1.into());
    }
}
