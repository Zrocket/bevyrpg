use std::time::Duration;
use bevy_trait_query::RegisterExt;

use super::GameState;
use super::utils::{F32Ext, Vec3Ext};
use crate::interact::Interaction;
use crate::{error_pipe, CollisionLayer, MiscItem, Player};
use avian3d::prelude::{ColliderConstructor, CollisionLayers, LayerMask, RigidBody};
use bevy::{gltf::Gltf, prelude::*};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use oxidized_navigation::{
    self, NavMesh, NavMeshSettings,
    debug_draw::{DrawNavMesh, OxidizedNavigationDebugDrawPlugin},
    query::{find_polygon_path, perform_string_pulling_on_path},
};
use oxidized_navigation::OxidizedNavigationPlugin;

#[derive(Resource)]
pub struct LevelGltf(pub Handle<Gltf>);

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

#[derive(Resource)]
pub struct _ActiveLevel(Handle<Gltf>);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct DesiredPosition(pub Vec3);

#[derive(Debug, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct Walk {
    /// Top speed on the ground
    pub speed: f32,
    /// Direction in which we want to walk and turn this tick.
    pub direction: Option<Dir3>,
}
impl Default for Walk {
    fn default() -> Self {
        Self {
            speed: 1.,
            direction: None,
        }
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    walk: Walk,
    tnua_conroller: TnuaController,
    float_height: FloatHeight,
}


#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
/// Must be larger than the height of the entity's center from the bottom of its
/// collider, or else the character will not float and Tnua will not work properly
pub struct FloatHeight(pub f32);

pub struct BlenderTranslationPlugin;
impl Plugin for BlenderTranslationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BlenderCollider>()
            .register_type::<BlenderBoxCollider>()
            .register_type::<BlenderAnimationName>()
            .register_type::<BlenderColliderConstructor>()
            .register_type::<BlenderProp>()
            .register_type::<BlenderNavmesh>()
            .register_type::<Walk>()
            .register_type::<FloatHeight>()
            .register_type::<DesiredPosition>()
            .add_systems(OnEnter(GameState::Gameplay), translate_components)
            .add_systems(OnEnter(GameState::Loading),gltf_preload)
            .add_systems(OnExit(GameState::Loading), animation_preload);

        /*app.add_plugins(OxidizedNavigationPlugin::<Collider>::new(
            NavMeshSettings::from_agent_and_bounds(0.5, 1.9, 250.0, -1.0),
        ));*/

        //app.add_plugins(OxidizedNavigationDebugDrawPlugin);
        app.add_plugins(TnuaAvian3dPlugin::new(Update));
        app.add_plugins(TnuaControllerPlugin::default());
       // app.add_systems(
            //Update,
           // (
                //toggle_nav_mesh_system,
                //navmesh_pathfinding.pipe(error_pipe),
                //apply_walking,
            //),
        //);
    }
}

fn translate_components(
    mut commands: Commands,
    prop_query: Query<Entity, With<BlenderProp>>,
    collider_query: Query<Entity, (With<BlenderColliderConstructor>, Without<BlenderProp>)>,
    animation_name_query: Query<Entity, With<BlenderAnimationName>>,
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

//
//  Toggle drawing Nav-mesh.
//  Press M to toggle drawing the navmesh.
//
fn toggle_nav_mesh_system(keys: Res<ButtonInput<KeyCode>>, mut show_navmesh: ResMut<DrawNavMesh>) {
    trace!("SYSTEM: toggle_nav_mesh");

    if keys.just_pressed(KeyCode::KeyM) {
        show_navmesh.0 = !show_navmesh.0;
    }
}

fn navmesh_pathfinding(
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    mut query: Query<(&Transform, &DesiredPosition, &mut Walk)>,
) -> anyhow::Result<()> {
    trace!("SYSTEM: navmesh_pathfinding");

    for (transform, desired_position, mut walk) in &mut query {
        if let Ok(nav_mesh) = nav_mesh.get().read() {
            if let Ok(path) = find_polygon_path(
                &nav_mesh,
                &nav_mesh_settings,
                transform.translation,
                desired_position.0,
                None,
                Some(&[1.0, 0.5]),
            ) {
                let path = perform_string_pulling_on_path(
                    &nav_mesh,
                    transform.translation,
                    desired_position.0,
                    &path,
                )
                .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?;

                let dir = path
                    .into_iter()
                    .map(|next_point| (next_point - transform.translation).horizontal())
                    .filter(|dir| dir.length_squared() > 1e-3f32.squared())
                    .filter_map(|dir| dir.try_normalize())
                    .next();
                if let Some(dir) = dir {
                    let dir = Dir3::new(dir);
                    if dir.is_ok() {
                        walk.direction = Some(dir.unwrap());
                    }
                };
            }
        }
    }
    Ok(())
}

fn apply_walking(
    mut character_query: Query<(&mut TnuaController, &mut Walk, &FloatHeight), Without<Player>>,
) {
    trace!("SYSTEM: apply_walking");

    for (mut controller, mut walking, float_height) in &mut character_query {
        if let Some(direction) = walking.direction {
            let speed = walking.speed;
            controller.basis(TnuaBuiltinWalk {
                desired_velocity: direction * speed,
                desired_forward: Some(direction),
                float_height: float_height.0,
                cling_distance: 1.0,
                ..Default::default()
            });
            walking.direction = None;
        }
    }
}

fn gltf_preload(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    trace!("SYSTEM: animation_preload");
    let level_gltf = asset_server.load("levels/World.glb");
    commands.insert_resource(LevelGltf(level_gltf));
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
