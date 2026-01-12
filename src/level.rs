use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt}, standard_dynamic_asset::StandardDynamicAssetCollection};
use bevy_sun_move::{SkyCenter, SunMovePlugin, TimedSkyConfig, random_stars::{RandomStarsPlugin, StarSpawner}};

use super::GameState;
use crate::{Climbable, LadderComponent, MiscItem, Obstacle, ladder_collision_observer, ladder_decollision_observer};
use avian3d::{prelude::{ColliderConstructor, CollidingEntities, CollisionEventsEnabled, CollisionLayers, LayerMask, PhysicsLayer, RigidBody}};
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

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct BlenderTranslationComplete;

#[derive(Message)]
pub struct ChangeLevelMessage(pub String);

#[derive(AssetCollection, Reflect, Resource, Debug)]
#[reflect(Resource)]
pub struct DALevelAsset {
    #[asset(key = "level")]
    level: Handle<Gltf>,
}

#[derive(AssetCollection, Reflect, Resource, Debug)]
#[reflect(Resource)]
pub struct DAGunAssets {
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
        app
            .add_plugins(SunMovePlugin)
            .add_plugins(RandomStarsPlugin)
            .register_type::<BlenderCollider>()
            .register_type::<BlenderBoxCollider>()
            .register_type::<BlenderAnimationName>()
            .register_type::<BlenderAnimations>()
            .register_type::<BlenderColliderConstructor>()
            .register_type::<BlenderProp>()
            .register_type::<BlenderNavmesh>()
            .register_type::<DALevelAsset>()
            .register_type::<DAGunAssets>()
            .register_type::<CollisionLayer>()
            .add_message::<ChangeLevelMessage>()
            .add_systems(Update, translate_components)
            .add_systems(Update, change_level_message_handler)
            .add_systems(OnExit(GameState::Loading), animation_preload.run_if(resource_added::<LevelGltf>))
            .add_loading_state(
                LoadingState::new(GameState::Preload)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("gunassets.ron")
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("devroom.ron")
                    //.with_dynamic_assets_file::<StandardDynamicAssetCollection>("fpslevel.ron")
                    .load_collection::<DAGunAssets>()
                    .load_collection::<DALevelAsset>()
            );
    }
}

fn change_level_message_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut change_level_messages: MessageReader<ChangeLevelMessage>,
    current_level_query: Query<Entity, With<CurrentLevel>>,
) {
    trace!("SYSTEM: change_level_message_handler");
    for message in change_level_messages.read() {
        if let Ok(current_level) = current_level_query.single() {
            commands.entity(current_level).despawn();
        }
        let level_gltf: Handle<Gltf> = asset_server.load(&message.0);
        commands.insert_resource(LevelGltf(level_gltf));

        let temp = message.0.clone();

        commands.spawn((
            SceneRoot(asset_server.load(
                    GltfAssetLabel::Scene(0).from_asset(temp),
            )),
            CurrentLevel,
        ));

        /*commands.spawn((
            DirectionalLight {
                //illuminance: light_consts::lux::OVERCAST_DAY,
                shadows_enabled: true,
                ..default()
            },
            Transform {
                translation: Vec3::new(0.0, 200.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
        ));*/

        let sun_id = commands.spawn((
                DirectionalLight {
                    shadows_enabled: true,
                    illuminance: light_consts::lux::RAW_SUNLIGHT, // Adjust illuminance as needed
                    ..default()
                },
                Transform::default(),
        ))
        .id();

        let timed_sky_config = TimedSkyConfig {
            sun_entity: sun_id,
            day_duration_secs: 10.0, // 10 seconds of dadylight
            night_duration_secs: 5.0, // 5 seconds of nighttime (15s total cycle)
            max_sun_height_deg: 60.0, // Sun reaches 60 degrees at noon
            planet_tilt_degrees: 23.5, // Earth's tilt (default)
            ..default()
        };

        // Calculate  and spawn the SkyCenter
        if let Some(sky_center) = SkyCenter::from_timed_config(&timed_sky_config) {
            commands.spawn((
                    sky_center,
                    // Optional: Add StarSpawner if you want the built-in stars
                    StarSpawner {
                        star_count: 1000,
                        spawn_radius: 5000.0, // Star distance
                    },
            ));
        } else {
            // Handle case where calculation failed (e.g., impossible parameters)
            error!("Failed to create SkyCenter from timed config");
        }

    }
}

fn translate_components(
    mut commands: Commands,
    prop_query: Query<Entity, (With<BlenderProp>, Without<BlenderTranslationComplete>)>,
    collider_query: Query<Entity, (With<BlenderColliderConstructor>, Without<BlenderProp>, Without<BlenderTranslationComplete>)>,
    ladder_query: Query<Entity, (With<LadderComponent>, Without<BlenderTranslationComplete>)>,
    mut loaded: Local<bool>,
) {
    trace!("SYSTEM: translate_blender_components");

    //if *loaded {
    //    return;
   // }

    for entity in prop_query.iter() {
        commands
            .entity(entity)
            .queue_silenced(|mut entity: EntityWorldMut| {
                entity
                    .insert(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
                    .insert(RigidBody::Dynamic)
                    .insert(MiscItem)
                    .insert(ColliderConstructor::ConvexHullFromMesh)
                    .insert(BlenderTranslationComplete);
            });
    }
    for entity in collider_query.iter() {
        commands.entity(entity)
            .queue_silenced(|mut entity: EntityWorldMut| {
                entity
                    .insert(RigidBody::Static)
                    .insert(Obstacle)
                    .insert(ColliderConstructor::ConvexHullFromMesh)
                    .insert(BlenderTranslationComplete);
            });
    }
    for entity in ladder_query.iter() {
        commands.entity(entity)
            .queue_silenced(|mut entity: EntityWorldMut| {
                entity
                    .insert(CollidingEntities::default())
                    .insert(CollisionEventsEnabled)
                    .insert(BlenderTranslationComplete)
                    .insert(Climbable)
                    .observe(ladder_collision_observer)
                    .observe(ladder_decollision_observer);
            });
    }

    *loaded = true;
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
            let animation_clip = animation_clip_handle.clone();
            let (animation_graph, _index) = AnimationGraph::from_clip(animation_clip);
            commands.entity(entity).insert(AnimationGraphHandle(animation_graphs.add(animation_graph)));
        }
    }
}
