use bevy::{ecs::{lifecycle::HookContext, system::SystemParam, world::DeferredWorld}, prelude::*};

use crate::{CloseDoorEvent, DoorState, OpenDoorEvent, level::LevelGltf};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    crate::Interactable,
)]
#[component(on_add = on_garage_button_add)]
pub struct GarageButton;

#[derive(SystemParam)]
pub struct GarageSystemParam<'w, 's> {
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
    animation_player_query: Query<'w, 's, &'static mut AnimationPlayer>,
    state_query: Query<'w, 's, (&'static mut DoorState, &'static ChildOf)>,
}

impl <'w, 's> GarageSystemParam<'w, 's> {
    fn open_garage(&mut self, entity: Entity) {
        trace!("SystemParam: open_garage");
        let Ok((mut door_state, child_of)) = self.state_query.get_mut(entity) else {
            error!("open_garage: Failed to query DoorState, ChildOf for {}", entity);
            return;
        };
        let parent_object = child_of.0;
        let Ok(mut animation_player) = self.animation_player_query.get_mut(parent_object) else {
            error!("open_garage: Failed to query AnimationPlayer for {}", parent_object);
            return;
        };
        animation_player.stop_all();
        println!("{:?}", door_state);
        animation_player.play(1.into());
        *door_state = DoorState::Open;
    }

    fn close_garage(&mut self, entity: Entity) {
        trace!("SystemParam: close_garage");
        let Ok((mut door_state, child_of)) = self.state_query.get_mut(entity) else {
            error!("close_garage: Failed to query DoorState for {}", entity);
            return;
        };
        let parent_object = child_of.0;
        let Ok(mut animation_player) = self.animation_player_query.get_mut(parent_object) else {
            error!("close_door_observer: Failed to query AnimationPlayer for {}", parent_object);
            return;
        };

        animation_player.stop_all();
        println!("{:?}", door_state);
        animation_player.play(2.into());
        *door_state = DoorState::Closed;
    }
}

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
    } else {
        error!("garage_button_interaction_observer: Failed to query GarageDoor's Entity and DoorState");
    }
}

fn open_garage_observer(
    trigger: On<OpenDoorEvent>,
    mut garage_commands: GarageSystemParam,
    //mut commands: Commands,
    //mut door_state_query: Query<&mut DoorState>,
    //asset_server: Res<AssetServer>,
    //child_of_query: Query<&ChildOf>,
    //mut door: Query<&mut AnimationPlayer>,
) {
    trace!("OBSERVER: open_door_observer");
    garage_commands.open_garage(trigger.entity);
    /*let Ok(mut door_state) = door_state_query.get_mut(trigger.entity) else {
        error!("open_garage_observer: Failed to query DoorState for {}", trigger.entity);
        return;
    };
    let Ok(child_of) = child_of_query.get(trigger.entity) else {
        error!("open_garage_observer: Failed to query ChildOf for {}", trigger.entity);
        return;
    };
    let parent_object = child_of.0;
    let Ok(mut door_animation_player) = door.get_mut(parent_object) else {
        error!("open_garage_observer: Failed to query AnimationPlayer for {}", parent_object);
        return;
    };

    door_animation_player.stop_all();
    println!("{:?}", door_state);
    door_animation_player.play(1.into());
    *door_state = DoorState::Open;*/
}

fn close_garage_observer(
    trigger: On<CloseDoorEvent>,
    mut garage_commands: GarageSystemParam,
    //mut commands: Commands,
    //mut door_state_query: Query<&mut DoorState>,
    //asset_server: Res<AssetServer>,
    //child_of_query: Query<&ChildOf>,
    //mut door: Query<&mut AnimationPlayer>,
) {
    trace!("OBSERVER: close_door_observer");
    garage_commands.close_garage(trigger.entity);
    /*let Ok(mut door_state) = door_state_query.get_mut(trigger.entity) else {
        error!("close_garage_observer: Failed to query DoorState for {}", trigger.entity);
        return;
    };
    let Ok(child_of) = child_of_query.get(trigger.entity) else {
        error!("close_garage_observer: Failed to query ChildOf for {}", trigger.entity);
        return;
    };
    let parent_object = child_of.0;
    let Ok(mut door_animation_player) = door.get_mut(parent_object) else {
        error!("close_door_observer: Failed to query AnimationPlayer for {}", parent_object);
        return;
    };

    door_animation_player.stop_all();
    println!("{:?}", door_state);
    door_animation_player.play(2.into());
    *door_state = DoorState::Closed;*/
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
    } else {
        error!("on_garage_door_add: Failed to query GLTF {:?}", level_gltf.0);
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
