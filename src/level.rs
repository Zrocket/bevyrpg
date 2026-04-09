use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt}, standard_dynamic_asset::StandardDynamicAssetCollection};
//use bevy_sun_move::{SkyCenter, TimedSkyConfig, random_stars::StarSpawner};

use crate::{BootStrap, MiscItem, Obstacle};
use avian3d::{prelude::{ColliderConstructor, CollisionLayers, LayerMask, PhysicsLayer, RigidBody}};
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
#[type_path("api")]
pub struct BlenderAnimations(pub Vec<String>);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[type_path("api")]
pub struct BlenderAnimationName(pub String);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[require(
    ColliderConstructor::ConvexHullFromMesh,
    BlenderTranslationComplete,
)]
#[type_path("api")]
pub struct BlenderCollider;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[require(
    CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL),
    RigidBody::Dynamic,
    MiscItem,
    ColliderConstructor::ConvexHullFromMesh,
    BlenderTranslationComplete,
)]
#[type_path("api")]
pub struct BlenderProp;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[type_path("api")]
pub struct BlenderBoxCollider {
    pub size: i32,
}

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
#[require(
    RigidBody::Static,
    Obstacle,
    ColliderConstructor::ConvexHullFromMesh,
    BlenderTranslationComplete,
)]
#[type_path("api")]
pub struct BlenderColliderConstructor;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[type_path("api")]
pub struct BlenderNavmesh;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[type_path("api")]
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
            .add_systems(Update, change_level_message_handler)
            .add_loading_state(
                LoadingState::new(BootStrap::Preload)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("gunassets.ron")
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("devroom.ron")
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

        /*let sun_id = commands.spawn((
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
        }*/

    }
}
