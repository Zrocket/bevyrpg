use bevy::{asset::LoadState, input::common_conditions::input_just_pressed, prelude::*, tasks::IoTaskPool};

use crate::MetaState;

pub const SCENE_FILE_PATH: &str = "saves/world.scn.ron";

#[derive(Resource)]
pub struct PendingSaveLoad;

#[derive(Resource)]
struct PendingSceneSpawn;

#[derive(Message)]
pub struct LoadGameMessage;

#[derive(Message)]
pub struct SaveGameMessage;

pub struct MySavePlugin;
impl Plugin for MySavePlugin {
    fn build(&self, app: &mut App) {
       app
           .add_message::<LoadGameMessage>()
           .add_message::<SaveGameMessage>()
           .add_systems(Update, quick_save.run_if(input_just_pressed(KeyCode::F5)))
           .add_systems(Update, quick_load.run_if(input_just_pressed(KeyCode::F9)))
           .add_systems(PostUpdate, (
                   save_observer,
                   load_scene.run_if(in_state(MetaState::Gameplay)),
                   spawn_scene_when_reloaded.run_if(in_state(MetaState::Gameplay)),
                   check_pending_save_load.run_if(in_state(MetaState::Gameplay))
           ));
    }
}

fn quick_load(
    mut load_game_message_writer: MessageWriter<LoadGameMessage>,
) {
    load_game_message_writer.write(LoadGameMessage);
}

fn quick_save(
    mut save_game_message_writer: MessageWriter<SaveGameMessage>,
) {
    save_game_message_writer.write(SaveGameMessage);
}

fn save_observer(
    mut commands: Commands,
    mut save_game_message_reader: MessageReader<SaveGameMessage>,
) {
    for _message in save_game_message_reader.read() {
        commands.queue(save_scene);
    }
}

fn load_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut load_game_message_reader: MessageReader<LoadGameMessage>,
    player_query: Query<Entity, With<crate::Player>>,
    rover_query: Query<Entity, With<crate::Rover>>,
    scene_root_query: Query<Entity, With<DynamicSceneRoot>>,
) {
    for _message in load_game_message_reader.read() {
        for entity in player_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in rover_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in scene_root_query.iter() {
            commands.entity(entity).despawn();
        }

        asset_server.reload(SCENE_FILE_PATH);
        let scene = asset_server.load(SCENE_FILE_PATH);
        commands.spawn(DynamicSceneRoot(scene));
    }

}

fn spawn_scene_when_reloaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pending: Option<Res<PendingSceneSpawn>>,
    mut scene_events: MessageReader<AssetEvent<DynamicScene>>,
) {
    if pending.is_none() { return; }

    for event in scene_events.read() {
        match event {
            AssetEvent::Modified { .. } | AssetEvent::LoadedWithDependencies { .. } => {
                let scene = asset_server.load(SCENE_FILE_PATH);
                commands.spawn(DynamicSceneRoot(scene));
                commands.remove_resource::<PendingSceneSpawn>();
                break;
            }
            _ => {}
        }
    }
}

fn save_scene(world: &mut World) {
    let mut player_query = world.query::<(Entity, &crate::Player)>();
    let mut rover_query = world.query::<(Entity, &crate::Rover)>();
    let mut items_query = world.query::<(Entity, &crate::InInventory)>();
    //let mut cam_query = world.query::<(Entity, &crate::PlayerCamera)>();
    let player_entities: Vec<Entity> = player_query.iter(world).map(|t| t.0).collect();
    let rover_entities: Vec<Entity> = rover_query.iter(world).map(|t| t.0).collect();
    let item_entities: Vec<Entity> = items_query.iter(world).map(|t| t.0).collect();
    //let cam_entities: Vec<Entity> = cam_query.iter(world).map(|t| t.0).collect();

    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let registry = type_registry.read();

    let scene = DynamicSceneBuilder::from_world(world)
        .deny_all()
        .allow_component::<Name>()
        //.allow_component::<GlobalTransform>()
        .allow_component::<Transform>()
        .allow_component::<crate::Player>()
        .allow_component::<crate::Health>()
        .allow_component::<crate::Mana>()
        .allow_component::<crate::Sleep>()
        .allow_component::<crate::Hunger>()
        .allow_component::<crate::Thirst>()
        .allow_component::<crate::Rover>()
        .allow_component::<crate::InInventory>()
        //.allow_component::<crate::PlayerCamera>()
        //.allow_component::<crate::RenderPlayer>()
        .extract_entities(player_entities.into_iter())
        .extract_entities(rover_entities.into_iter())
        .extract_entities(item_entities.into_iter())
        //.extract_entities(cam_entities.into_iter())
        .extract_resources()
        .remove_empty_entities()
        .build();

    let serialized_scene = scene.serialize(&registry).unwrap();

    info!("{}", serialized_scene);

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            use std::{fs::File, io::Write};
            File::create(format!("assets/{SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
    })
    .detach();
}

fn check_pending_save_load(
    mut commands: Commands,
    pending: Option<Res<PendingSaveLoad>>,
    asset_server: Res<AssetServer>,
    level_gltf: Option<Res<crate::LevelGltf>>,
    mut load_game_message_writer: MessageWriter<crate::LoadGameMessage>,
) {
    if pending.is_none() { return; }


    if let Some(level_gltf) = level_gltf {
        if let LoadState::Loaded = asset_server.load_state(&level_gltf.0) {
            load_game_message_writer.write(crate::LoadGameMessage);
            commands.remove_resource::<PendingSaveLoad>();
        }
        //if asset_server.load_state(&level_gltf.0) == LoadState::Loaded {
        //match asset_server.load_state(&level_gltf.0) {
        //    LoadState::Loaded => {
        //        load_game_message_writer.write(crate::LoadGameMessage);
        //        commands.remove_resource::<PendingSaveLoad>();
        //    },
        //    _ => {}
        //}
    }
}
