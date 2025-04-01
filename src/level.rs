use super::utils::{F32Ext, Vec3Ext};
use super::GameState;
use crate::{error_pipe, MeshExt, Player};
use avian3d::collision::Collider;
use bevy::{gltf::Gltf, prelude::*};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use oxidized_navigation::{
    self,
    debug_draw::{DrawNavMesh, OxidizedNavigationDebugDrawPlugin},
    query::{find_polygon_path, perform_string_pulling_on_path},
    NavMesh, NavMeshSettings,
};
use oxidized_navigation::{NavMeshAffector, OxidizedNavigationPlugin};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct BlenderCollider;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct BlenderBoxCollider {
    pub size: i32,
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct BlenderNavmesh;

#[derive(Resource)]
pub struct ActiveLevel(Handle<Gltf>);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct DesiredPosition(pub Vec3);

#[derive(Debug, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct Walk {
    /// Top speed on the ground
    pub speed: f32,
    /// Direction in which we want to walk and turn this tick.
    //pub direction: Option<Vec3>,
    pub direction: Option<Dir3>,
}

#[derive(Bundle)]
pub struct MovementBundle {
    walk: Walk,
    //tnua_rapier3d_io: TnuaRapier3dIOBundle,
    tnua_conroller: TnuaController,
    float_height: FloatHeight,
}

impl Default for Walk {
    fn default() -> Self {
        Self {
            speed: 1.,
            direction: None,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
/// Must be larger than the height of the entity's center from the bottom of its
/// collider, or else the character will not float and Tnua will not work properly
pub struct FloatHeight(pub f32);

//#[derive(Resource)]
//pub struct ActiveNavMesh(Handle<NavMesh>);

//#[derive(Resource)]
//struct Handles(Handle<Gltf>, Option<Handle<NavMesh>>);

pub struct BlenderTranslationPlugin;

impl Plugin for BlenderTranslationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BlenderCollider>()
            .register_type::<BlenderBoxCollider>()
            .register_type::<BlenderNavmesh>()
            .register_type::<Walk>()
            .register_type::<FloatHeight>()
            .register_type::<DesiredPosition>()
            .add_systems(OnEnter(GameState::Gameplay), translate_components);

        app.add_plugins(OxidizedNavigationPlugin::<Collider>::new(
            NavMeshSettings::from_agent_and_bounds(0.5, 1.9, 250.0, -1.0),
        ));

        app.add_plugins(OxidizedNavigationDebugDrawPlugin);
        app.add_plugins(TnuaAvian3dPlugin::new(Update));
        app.add_plugins(TnuaControllerPlugin::default());
        app.add_systems(
            Update,
            (
                //run_blocking_pathfinding,
                toggle_nav_mesh_system,
                navmesh_pathfinding.pipe(error_pipe),
                apply_walking,
            ),
        );
    }
}

fn translate_components(
    mut commands: Commands,
    query: Query<Entity, With<BlenderCollider>>,
    //box_query: Query<(Entity, &BlenderBoxCollider)>,
    meshes: Res<Assets<Mesh>>,
    //mesh_handles: Query<&Handle<Mesh>>,
    mesh_handles: Query<&Mesh3d>,
    children: Query<&Children>,
) {
    //info!("Translate Components");
    for entity in query.iter() {
        //info!("Translate Event");
        //for (_, collider_mesh) in Mesh::search_in_children(entity, &children, &meshes, &mesh_handles) {
        for (_, collider_mesh) in
            Mesh::search_in_children(entity, &children, &meshes, &mesh_handles)
        {
            //info!("Translate Components 2");
            //let rapier_collider = Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh)
            let avian_collider = Collider::trimesh_from_mesh(collider_mesh).unwrap();
            commands
                .entity(entity)
                .insert(avian_collider)
                .insert(NavMeshAffector);
        }
    }
    //for (entity, box_collider) in box_query.iter() {
    //    !todo!()
    //}
}

//
//  Toggle drawing Nav-mesh.
//  Press M to toggle drawing the navmesh.
//
fn toggle_nav_mesh_system(keys: Res<ButtonInput<KeyCode>>, mut show_navmesh: ResMut<DrawNavMesh>) {
    if keys.just_pressed(KeyCode::KeyM) {
        show_navmesh.0 = !show_navmesh.0;
    }
}

fn navmesh_pathfinding(
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    mut query: Query<(&Transform, &DesiredPosition, &mut Walk)>,
) -> anyhow::Result<()> {
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
                //commands.spawn(DrawPath {
                //    timer: Some(Timer::from_seconds(4.0, TimerMode::Once)),
                //    pulled_path: path,
                //    color: palettes::css::RED.into(),
                //});

                let dir = path
                    .into_iter()
                    .map(|next_point| (next_point - transform.translation).horizontal())
                    .filter(|dir| dir.length_squared() > 1e-3f32.squared())
                    .filter_map(|dir| dir.try_normalize())
                    .next();
                let dir = Dir3::new(dir.unwrap()).unwrap();
                walk.direction = Some(dir);
            }
        }
    }
    Ok(())
}

fn apply_walking(
    mut character_query: Query<(&mut TnuaController, &mut Walk, &FloatHeight), Without<Player>>,
) {
    for (mut controller, mut walking, float_height) in &mut character_query {
        //let direction = walking.direction.unwrap_or_default();
        if let Some(direction) = walking.direction {
            let speed = walking.speed;
            controller.basis(TnuaBuiltinWalk {
                desired_velocity: direction * speed,
                //desired_forward: direction.normalize_or_zero(),
                desired_forward: Some(direction),
                float_height: float_height.0,
                cling_distance: 1.0,
                ..Default::default()
            });
            walking.direction = None;
        }
        //let direction = walking.direction.unwrap();
        //let sprinting_multiplier = sprinting
        //    .filter(|s| s.requested)
        //    .map(|s| s.multiplier)
        //    .unwrap_or(1.);
        //let speed = walking.speed * sprinting_multiplier;
    }
}
