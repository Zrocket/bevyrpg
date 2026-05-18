use std::{fs, path};

use bevy::{asset::LoadState, input::common_conditions::input_just_pressed, prelude::*, tasks::IoTaskPool};

use crate::MetaState;
use crate::load_game::SaveRef;

pub const SCENE_FILE_PATH: &str = "saves/world.scn.ron";

#[derive(Resource)]
pub struct PendingSave(pub Entity);

#[derive(Resource)]
pub struct PendingSaveLoad(pub Entity);

#[derive(Resource)]
struct PendingSceneSpawn;

#[derive(Message)]
pub struct LoadGameMessage;

#[derive(Message)]
pub struct SaveGameMessage;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SaveFile(pub String);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct SaveStorage(pub Vec<Entity>);

pub struct MySavePlugin;
impl Plugin for MySavePlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<SaveStorage>()
           .register_type::<SaveFile>()
           .add_message::<LoadGameMessage>()
           .add_message::<SaveGameMessage>()
           .add_systems(Update, quick_save.run_if(input_just_pressed(KeyCode::F5)))
           .add_systems(Update, quick_load.run_if(input_just_pressed(KeyCode::F9)))
           //.add_systems(Update, initialize_save_files.run_if(input_just_pressed(KeyCode::F1)))
           .add_systems(Startup, initialize_save_files)
           .add_systems(PostUpdate, (
                   save_observer,
                   load_scene.run_if(in_state(MetaState::Gameplay)),
                   spawn_scene_when_reloaded.run_if(in_state(MetaState::Gameplay)),
                   check_pending_load.run_if(in_state(MetaState::Gameplay))
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

fn initialize_save_files(
    mut commands: Commands,
) {
    let tmp = format!("assets/saves/");
    let path = path::Path::new(&tmp);
    if let Ok(dir) = fs::read_dir(path) {
        let mut save_files: Vec<Entity> = Vec::new();
        for file in dir {
            if let Ok(file) = file {
                if let Ok(file_name) = file.file_name().into_string() {
                    println!("{:?}", file_name);
                    let save = commands.spawn(SaveFile(file_name.clone())).id();
                    save_files.push(save.clone());
                }
            }
            commands.insert_resource(SaveStorage(save_files.clone()));
        }
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


#[allow(clippy::too_many_arguments)]
fn check_pending_load(
    mut commands: Commands,
    pending: Option<Res<PendingSaveLoad>>,
    asset_server: Res<AssetServer>,
    level_gltf: Option<Res<crate::LevelGltf>>,
    player_query: Query<Entity, With<crate::Player>>,
    rover_query: Query<Entity, With<crate::Rover>>,
    scene_root_query: Query<Entity, With<DynamicSceneRoot>>,
    save_file_query: Query<&SaveFile>,
) {
    if pending.is_none() { return; }
    let pending = pending.unwrap();

    if let Some(level_gltf) = level_gltf
    && let Ok(save) = save_file_query.get(pending.0)
    && let LoadState::Loaded = asset_server.load_state(&level_gltf.0) {
        for entity in player_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in rover_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in scene_root_query.iter() {
            commands.entity(entity).despawn();
        }

        let file = save.0.clone();
        let file = format!("saves/{file}");

        let scene = asset_server.load(file);

        commands.spawn(DynamicSceneRoot(scene));
        commands.remove_resource::<PendingSaveLoad>();
    }
}

#[allow(clippy::too_many_arguments)]
fn check_pending_save(
    mut commands: Commands,
    pending: Option<Res<PendingSave>>,
    asset_server: Res<AssetServer>,
    level_gltf: Option<Res<crate::LevelGltf>>,
    save_file_query: Query<&SaveFile>,
) {
    if pending.is_none() { return; }
    let pending = pending.unwrap();

    if let Some(level_gltf) = level_gltf
    && let Ok(save) = save_file_query.get(pending.0) {
        let file = save.0.clone();
        let file = format!("saves/{file}");

        commands.remove_resource::<PendingSave>();
    }
}
