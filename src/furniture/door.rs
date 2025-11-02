use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::interact::Interaction;

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

#[derive(Event)]
pub struct DoorEvent {
    actor: Entity,
    target: Entity,
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct DoorComponent;
impl Interaction for DoorComponent {
    fn interact(&self,commands: &mut Commands,entity:Entity,prop:Entity,) {
        println!("Door Interaction");
        commands.trigger(DoorEvent {actor: entity, target: prop});
    }
}

pub struct DoorPlugin;
impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DoorComponent>()
            .register_component_as::<dyn Interaction, DoorComponent>()
            //.add_event::<DoorEvent>()
            .add_observer(door_event_observer);
    }
}

fn door_event_observer(
    _trigger: On<DoorEvent>,
    mut door: Query<(Entity, &mut AnimationPlayer)>,
) {
    println!("CCCCCCCCCCC");
    trace!("OBSERVER: door_event_observer");
    if let Ok((_door_entity, mut door_animation_player)) = door.single_mut() {
        door_animation_player.play(1.into());
    }
}
