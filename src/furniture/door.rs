use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_seedling::sample::SamplePlayer;
use rand::Rng;

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

pub struct DoorPlugin;
impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DoorComponent>()
            .register_type::<DoorState>()
            .register_type::<LockedState>();
            //.init_resource::<DoorGraph>();
    }
}

fn on_door_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_door_add");

    //let graph = world.resource::<DoorGraph>().0.clone();

    let level_gltf = world.resource::<LevelGltf>();

    if let Some(gltf) = world.resource::<Assets<Gltf>>().get(&level_gltf.0) {
        let open_animation_clip_handle = gltf.named_animations["opendoor"].clone();
        let close_animation_clip_handle = gltf.named_animations["closedoor"].clone();
        //let (animation_graph, _index) = AnimationGraph::from_clip(open_animation_clip_handle);
        let (animation_graph, _index) = AnimationGraph::from_clips([open_animation_clip_handle, close_animation_clip_handle]);
        let mut animation_graphs = world.resource_mut::<Assets<AnimationGraph>>();
        let graph = AnimationGraphHandle(animation_graphs.add(animation_graph));

    world.commands().entity(context.entity)
        .insert(graph);
    }

    world.commands()
        .entity(context.entity)
        .observe(door_interaction_observer);
}

fn door_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    child_of_query: Query<&ChildOf>,
    mut door: Query<(Entity, &mut AnimationPlayer, &mut DoorState, Option<&LockedState>)>,
) {
    trace!("OBSERVER: door_event_observer");
    if let Ok(child_of) = child_of_query.get(trigger.entity)
    && let Ok(parent_child_of) = child_of_query.get(child_of.0)
    && let Ok(parent_parent_child_of) = child_of_query.get(parent_child_of.0)
    && let Ok((_door_entity, mut door_animation_player, mut door_state, locked_state)) = door.get_mut(parent_parent_child_of.0) {
        if locked_state.is_some()
        && locked_state.unwrap() == &LockedState::Locked {
            return;
        }
        let mut rng = rand::rng();
        let file = format!("audio/door/qubodup-DoorOpen0{}.ogg", rng.random_range(0..8));
        if *door_state == DoorState::Closed {
            door_animation_player.stop_all();
            door_animation_player.play(1.into());
            *door_state = DoorState::Open;
            commands.spawn(
                SamplePlayer::new(asset_server.load(file))
            );
        } else {
            door_animation_player.stop_all();
            door_animation_player.play(2.into());
            *door_state = DoorState::Closed;
            commands.spawn(
                SamplePlayer::new(asset_server.load(file))
            );
        }
    }
}
