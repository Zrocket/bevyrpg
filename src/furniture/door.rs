use avian3d::prelude::{ColliderConstructor, RigidBody};
use bevy::{ecs::{event::Trigger, lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_seedling::sample::SamplePlayer;
use rand::random_range;

use crate::{Interactable, InteractionEvent, level::{LevelGltf}};

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, Component, Reflect)]
#[reflect(Component)]
pub enum DoorState {
    #[default]
    Closed,
    Open,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, Component, Reflect)]
#[reflect(Component)]
pub enum LockedState {
    #[default]
    Unlocked,
    Locked,
}

#[derive(Resource)]
pub struct OpenDoorAnimation(pub Handle<AnimationClip>);

/*impl FromWorld for OpenDoorAnimation {
    fn from_world(world: &mut World) -> Self {
    }
}*/

#[derive(Resource)]
pub struct CloseDoorAnimation(pub Handle<AnimationClip>);

/*impl FromWorld for CloseDoorAnimation {
    fn from_world(world: &mut World) -> Self {
        let level_gltf = world.resource::<LevelGltf>();
        if let Some(gltf) = world.resource::<Assets<Gltf>>().get(&level_gltf.0) {
            let close_animation_clip_handle = gltf.named_animations["closedoor"].clone();
            Self(close_animation_clip_handle)
        } else {
            Self(Handle<AnimationClip::default()>)
        }
    }
}*/

#[derive(Resource)]
pub struct DoorGraph(pub AnimationGraphHandle);

/*impl FromWorld for DoorGraph {
    fn from_world(world: &mut World) -> Self {
        let level_gltf = world.resource::<LevelGltf>();
        if let Some(gltf) = world.resource::<Assets<Gltf>>().get(&level_gltf.0) {
            let open_animation_clip_handle = gltf.named_animations["opendoor"].clone();
            let close_animation_clip_handle = gltf.named_animations["closedoor"].clone();
            let (animation_graph, _index) = AnimationGraph::from_clips([open_animation_clip_handle, close_animation_clip_handle]);

            let mut animation_graphs = world.resource_mut::<Assets<AnimationGraph>>();
            let graph = AnimationGraphHandle(animation_graphs.add(animation_graph));

            Self(graph)
        } else {
            Self(AnimationGraphHandle::default())
        }
    }
}*/

#[derive(Component)]
#[relationship(relationship_target = Keys)]
pub struct KeyOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = KeyOf)]
pub struct Keys(Vec<Entity>);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_door_add)]
#[require(
    Interactable,
    DoorState,
)]
#[type_path("api")]
pub struct DoorComponent;

#[derive(EntityEvent)]
pub struct OpenDoorEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct CloseDoorEvent {
    pub entity: Entity,
}

pub struct DoorPlugin;
impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DoorComponent>()
            .register_type::<DoorState>()
            .register_type::<LockedState>();
    }
}

fn close_door_observer(
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
        let file = format!("audio/door/qubodup-DoorOpen0{}.ogg", random_range(0..8));
        println!("{:?}", door_state);
        door_animation_player.play(2.into());
        commands.spawn(
            SamplePlayer::new(asset_server.load(file))
        );
        *door_state = DoorState::Closed;
    }
}

fn open_door_observer(
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
        let file = format!("audio/door/qubodup-DoorOpen0{}.ogg", random_range(0..8));
        println!("{:?}", door_state);
        door_animation_player.play(1.into());
        commands.spawn(
            SamplePlayer::new(asset_server.load(file))
        );
        *door_state = DoorState::Open;
    }
}

fn on_door_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_door_add");

    let level_gltf = world.resource::<LevelGltf>();

    if let Some(gltf) = world.resource::<Assets<Gltf>>().get(&level_gltf.0) {
        let open_animation_clip_handle = gltf.named_animations["open"].clone();
        let close_animation_clip_handle = gltf.named_animations["close"].clone();
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

        world.commands()
            .entity(context.entity)
            .observe(door_interaction_observer)
            .observe(open_door_observer)
            .observe(close_door_observer);
    }
}

fn door_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
    mut door_state_query: Query<(&mut DoorState, Option<&LockedState>)>,
) {
    trace!("OBSERVER: door_event_observer");
    if let Ok((mut door_state, lock_state)) = door_state_query.get_mut(trigger.entity) {
        if *door_state == DoorState::Closed {
            if let Some(lock_state) = lock_state
            && *lock_state == LockedState::Locked {
                return;
            }
            println!("{:?}", door_state);
            commands.entity(trigger.entity).trigger(|entity| OpenDoorEvent { entity });
            *door_state = DoorState::Open;
        } else {
            println!("{:?}", door_state);
            commands.entity(trigger.entity).trigger(|entity| CloseDoorEvent { entity });
            *door_state = DoorState::Closed;
        }
    }
}
