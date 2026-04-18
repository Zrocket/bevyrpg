use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{CloseDoorEvent, DoorState, OpenDoorEvent, level::LevelGltf};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    crate::Interactable,
)]
#[component(on_add = on_garage_button_add)]
pub struct GarageButton;

fn on_garage_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(garage_button_interaction_observer);
}

fn garage_button_interaction_observer(
    _trigger: On<crate::InteractionEvent>,
    mut commands: Commands,
    mut garage_query: Query<(Entity, &mut DoorState), With<GarageDoor>>,
) {
    if let Ok((door, mut state)) = garage_query.single_mut() {
        if *state == DoorState::Closed {
            println!("{:?}", state);
            commands.entity(door).trigger(|entity| OpenDoorEvent { entity });
            *state = DoorState::Open;
        } else {
            println!("{:?}", state);
            commands.entity(door).trigger(|entity| CloseDoorEvent { entity });
            *state = DoorState::Closed;
        }
    }
}

fn open_garage_observer(
    trigger: On<OpenDoorEvent>,
    mut commands: Commands,
    mut door_state_query: Query<&mut DoorState>,
    asset_server: Res<AssetServer>,
    child_of_query: Query<&ChildOf>,
    mut door: Query<&mut AnimationPlayer>,
) {
    trace!("OBSERVER: open_door_observer");
    if let Ok(mut door_state) = door_state_query.get_mut(trigger.entity)
    && let Ok(child_of) = child_of_query.get(trigger.entity)
    && let parent_object = child_of.0
    && let Ok(mut door_animation_player) = door.get_mut(parent_object) {
        door_animation_player.stop_all();
        println!("{:?}", door_state);
        door_animation_player.play(1.into());
        *door_state = DoorState::Open;
    }
}

fn close_garage_observer(
    trigger: On<CloseDoorEvent>,
    mut commands: Commands,
    mut door_state_query: Query<&mut DoorState>,
    asset_server: Res<AssetServer>,
    child_of_query: Query<&ChildOf>,
    mut door: Query<&mut AnimationPlayer>,
) {
    trace!("OBSERVER: open_door_observer");
    if let Ok(mut door_state) = door_state_query.get_mut(trigger.entity)
    && let Ok(child_of) = child_of_query.get(trigger.entity)
    && let parent_object = child_of.0
    && let Ok(mut door_animation_player) = door.get_mut(parent_object) {
        door_animation_player.stop_all();
        println!("{:?}", door_state);
        door_animation_player.play(2.into());
        *door_state = DoorState::Closed;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    crate::DoorState,
)]
#[component(on_add = on_garage_door_add)]
pub struct GarageDoor;

fn on_garage_door_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_garage_door_add");

    let level_gltf = world.resource::<LevelGltf>();

    if let Some(gltf) = world.resource::<Assets<Gltf>>().get(&level_gltf.0) {
        let open_animation_clip_handle = gltf.named_animations["garage_open"].clone();
        let close_animation_clip_handle = gltf.named_animations["garage_close"].clone();
        let (animation_graph, _index) = AnimationGraph::from_clips([open_animation_clip_handle, close_animation_clip_handle]);
        let mut animation_graphs = world.resource_mut::<Assets<AnimationGraph>>();
        let graph = AnimationGraphHandle(animation_graphs.add(animation_graph));

        let parent = world
            .entity(context.entity)
            .get::<ChildOf>()
            .map(|c| c.0);

        if let Some(parent_entity) = parent {
            world.commands().entity(parent_entity).insert(graph);
        }
    }

    world.commands()
        .entity(context.entity)
        .observe(open_garage_observer)
        .observe(close_garage_observer);
}

pub struct GaragePlugin;
impl Plugin for GaragePlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<GarageDoor>()
           .register_type::<GarageButton>(); 
    }
}
