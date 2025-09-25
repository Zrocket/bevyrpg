use bevy::{asset::AssetPath, input::common_conditions::input_pressed};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt}, standard_dynamic_asset::StandardDynamicAssetCollection};

use super::GameState;
use crate::MiscItem;
use avian3d::prelude::{ColliderConstructor, CollisionLayers, LayerMask, Physics, PhysicsLayer, RigidBody};
use bevy::{gltf::Gltf, prelude::*};

#[derive(Debug, PhysicsLayer, Default, Component, Reflect)]
#[reflect(Component)]
pub enum CollisionLayer {
    #[default]
    Default,
    Player,
    Prop,
}

#[derive(Resource)]
pub struct LevelGltf(pub Handle<Gltf>);

#[derive(Component)]
pub struct CurrentLevel;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct BlenderAnimations(pub Vec<String>);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct BlenderAnimationName(pub String);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct BlenderCollider;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct BlenderProp;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct BlenderBoxCollider {
    pub size: i32,
}

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct BlenderColliderConstructor;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct BlenderNavmesh;

#[derive(Event)]
pub struct ChangeLevelEvent(String);

#[derive(AssetCollection, Reflect, Resource, Debug)]
#[reflect(Resource)]
pub struct DA_LevelAsset {
    #[asset(key = "level")]
    level: Handle<Gltf>,
}

#[derive(AssetCollection, Reflect, Resource, Debug)]
#[reflect(Resource)]
pub struct DA_GunAssets {
    #[asset(key = "uzi")]
    pub uzi: Handle<Gltf>,
    #[asset(key = "shotgun")]
    pub shotgun: Handle<Gltf>,
    #[asset(key = "sniper")]
    pub sniper: Handle<Gltf>,
    #[asset(key = "mp5")]
    pub mp5: Handle<Gltf>,
}

pub struct BlenderTranslationPlugin;
impl Plugin for BlenderTranslationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BlenderCollider>()
            .register_type::<BlenderBoxCollider>()
            .register_type::<BlenderAnimationName>()
            .register_type::<BlenderAnimations>()
            .register_type::<BlenderColliderConstructor>()
            .register_type::<BlenderProp>()
            .register_type::<BlenderNavmesh>()
            .register_type::<DA_LevelAsset>()
            .register_type::<DA_GunAssets>()
            .register_type::<CollisionLayer>()
            .add_event::<ChangeLevelEvent>()
            .add_systems(OnEnter(GameState::Gameplay), translate_components)
            .add_systems(OnEnter(GameState::Preload),gltf_preload)
            .add_systems(Update, test_reload.run_if(input_pressed(KeyCode::KeyR)))
            .add_systems(Update, change_level_event_handler)
            //.add_systems(Update, wait_for_level_load)
            .add_systems(OnExit(GameState::Loading), animation_preload)
            .add_loading_state(
                LoadingState::new(GameState::Preload)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("gunassets.ron")
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("devroom.ron")
                    //.with_dynamic_assets_file::<StandardDynamicAssetCollection>("fpslevel.ron")
                    .load_collection::<DA_GunAssets>()
                    .load_collection::<DA_LevelAsset>()
            );
    }
}

fn wait_for_level_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut time: ResMut<Time<Physics>>,
    //level_gltf: Res<LevelGltf>,
    level_gltf: Res<DA_LevelAsset>,
    gltf_assets: Res<Assets<Gltf>>,
    mut loaded: Local<bool>,
) {
    trace!("SYSTEM: wait_for_level_load");
    if *loaded {
        return;
    }
    let Some(gltf) = gltf_assets.get(&level_gltf.level) else {
        return;
    };
    commands.spawn(
(
            CurrentLevel,
            SceneRoot(gltf.named_scenes["World"].clone())
        )
    );
    *loaded = true;
}

fn change_level_event_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut time: ResMut<Time<Physics>>,
    mut change_level_events: EventReader<ChangeLevelEvent>,
    current_level_query: Query<Entity, With<CurrentLevel>>,
) {
    trace!("SYSTEM: change_level_event_handler");
    for event in change_level_events.read() {
        if let Ok(current_level) = current_level_query.single() {
            commands.entity(current_level).despawn();
        }
        let level_gltf: Handle<Gltf> = asset_server.load(&event.0);
        commands.insert_resource(LevelGltf(level_gltf));
    }
}

fn translate_components(
    mut commands: Commands,
    prop_query: Query<Entity, With<BlenderProp>>,
    collider_query: Query<Entity, (With<BlenderColliderConstructor>, Without<BlenderProp>)>,
) {
    trace!("SYSTEM: translate_blender_components");

    for entity in prop_query.iter() {
        commands
            .entity(entity)
            .insert(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
            .insert(RigidBody::Dynamic)
            .insert(MiscItem)
            .insert(ColliderConstructor::ConvexHullFromMesh);
    }
    for entity in collider_query.iter() {
        commands.entity(entity)
            .insert(RigidBody::Static)
            .insert(ColliderConstructor::ConvexHullFromMesh);
    }
}

fn gltf_preload(
    mut change_level_event_wriiter: EventWriter<ChangeLevelEvent>,
    //level_asset: Res<DA_LevelAsset>,
    //gltf_assets: Res<Assets<Gltf>>,
) {
    trace!("SYSTEM: gltf_preload");
    //let level = gltf_assets.get(&level_asset.level).unwrap();
    //change_level_event_wriiter.write(ChangeLevelEvent("levels/World.glb".into()));
    change_level_event_wriiter.write(ChangeLevelEvent("levels/World.glb".into()));
}

fn test_reload(
    mut change_level_event_wriiter: EventWriter<ChangeLevelEvent>,
) {
    trace!("SYSTEM: test_reload");
    change_level_event_wriiter.write(ChangeLevelEvent("levels/fps.glb".into()));
}

fn animation_preload(
    mut commands: Commands,
    level_gltf: Res<LevelGltf>,
    gltf_assets: Res<Assets<Gltf>>,
    blender_animation_query: Query<(Entity, &BlenderAnimationName), Without<AnimationGraphHandle>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
){
    trace!("SYSTEM: animation_preload");
    if let Some(gltf) = gltf_assets.get(&level_gltf.0) {
        for (entity, animation_name) in blender_animation_query.iter() {
            let animation_clip_handle = gltf.named_animations[animation_name.0.as_str()].clone();
            let animation_clip = animation_clip_handle.clone_weak();
            let (animation_graph, _index) = AnimationGraph::from_clip(animation_clip);
            commands.entity(entity).insert(AnimationGraphHandle(animation_graphs.add(animation_graph)));
        }
    }
}
