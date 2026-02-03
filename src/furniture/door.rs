use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_seedling::sample::SamplePlayer;
use rand::Rng;

use crate::{Interactable, InteractionEvent, level::{BlenderAnimationName, LevelGltf}};

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, Component, Reflect)]
#[reflect(Component)]
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
#[require(
    Interactable,
    DoorState,
)]
pub struct DoorComponent;

pub struct DoorPlugin;
impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DoorComponent>()
            .register_type::<DoorState>();
    }
}

fn on_door_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_door_add");
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
    mut door: Query<(Entity, &mut AnimationPlayer, &mut DoorState)>,
) {
    trace!("OBSERVER: door_event_observer");
    if let Ok(child_of) = child_of_query.get(trigger.entity)
    && let Ok(parent_child_of) = child_of_query.get(child_of.0)
    && let Ok(parent_parent_child_of) = child_of_query.get(parent_child_of.0)
    && let Ok((_door_entity, mut door_animation_player, mut door_state)) = door.get_mut(parent_parent_child_of.0) {
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
