use std::time::Duration;

use avian3d::{math::FRAC_PI_2, prelude::Collider};
//use avian_rerecast::AvianBackendPlugin;
use bevy::{asset::uuid_handle, camera::primitives::Aabb, color::palettes, prelude::*, time::common_conditions::on_timer};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use polyanya::Triangulation;
use vleue_navigator::{NavMeshDebug, VleueNavigatorPlugin, prelude::{ManagedNavMesh, NavMeshSettings, NavMeshUpdateMode, NavmeshUpdaterPlugin}};
//use bevy_rerecast::{debug::DetailNavmeshGizmo, prelude::*};

use crate::{GameState, Player};

//#[derive(Resource)]
//struct NavMeshHandle(Handle<Navmesh>);

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

#[derive(Debug, Component)]
pub struct Obstacle;

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
/// Must be larger than the height of the entity's center from the bottom of its
/// collider, or else the character will not float and Tnua will not work properly
pub struct FloatHeight(pub f32);

pub struct NavMeshPlugin;
impl Plugin for NavMeshPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(NavmeshPlugins::default())
            //.add_plugins(AvianBackendPlugin::default())
            .register_type::<Walk>()
            .register_type::<FloatHeight>()
            .register_type::<DesiredPosition>()
            .add_plugins(TnuaAvian3dPlugin::new(FixedUpdate))
            .add_plugins(TnuaControllerPlugin::new(FixedUpdate))
            .add_plugins(VleueNavigatorPlugin)
            .add_plugins(NavmeshUpdaterPlugin::<Collider, Obstacle>::default())
            .add_systems(OnEnter(GameState::Gameplay), vleue_navigator_setup)
            .add_systems(Update, view_navmesh.run_if(on_timer(Duration::from_secs_f32(1.0))));
            //.add_systems(OnEnter(GameState::Loading), generate_navmesh);
    }
}

pub const MATERIAL_OBSTACLE_LIVE: Handle<StandardMaterial> = uuid_handle!("369EA0F5-EC58-457E-94E4-BDB70D99AA20");
pub const MATERIAL_OBSTACLE_CACHED: Handle<StandardMaterial> = uuid_handle!("5895917E-BCB3-402B-AF02-39400A0D8F65");

fn vleue_navigator_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mushes: ResMut<Assets<Mesh>>,
) {
    let obstacle_size = 2.0;
    let spacing = 1.0;
    let nb_navmeshes = 3;
    let height_step = obstacle_size / (nb_navmeshes as f32);

    for idx in 0..nb_navmeshes {
        commands.spawn((
                ManagedNavMesh::from_id(idx as u128),
                NavMeshSettings {
                    // Define the outer borders of the navmesh.
                    fixed: Triangulation::from_outer_edges(&[
                               vec2(-25.0, -25.0),
                               vec2(25.0, -25.0),
                               vec2(25.0, 25.0),
                               vec2(-25.0, 25.0),
                    ]),
                    build_timeout: Some(1.0),
                    simplify: 0.005,
                    merge_steps: 0,
                    ..default()
                },
                NavMeshUpdateMode::Direct,
                Transform::from_xyz(0.0, idx as f32 * height_step + 0.1, 0.0)
                    .with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
        ));
    }
}

fn view_navmesh(
    mut commands: Commands,
    navmeshes: Query<Entity, With<ManagedNavMesh>>,
    mut current: Local<usize>,
) {
    if navmeshes.iter().len() == 0 {
        return;
    }
    for (i, entity) in navmeshes.iter().sort::<Entity>().enumerate() {
        commands.entity(entity).remove::<NavMeshDebug>();
        if i == *current {
            commands
                .entity(entity)
                .insert(NavMeshDebug(palettes::tailwind::RED_800.into()));
        }
    }
    *current = (*current + 1) % navmeshes.iter().len();
}

/*fn rerecast_generate_navmesh(
    mut generator: NavmeshGenerator,
    mut commands: Commands,
) {
    //let agent_radius = 0.6;
    //let agent_height = 1.8;
    //let settings = NavmeshSettings::from_agent_3d(agent_radius, agent_height);
    let settings = NavmeshSettings::default();
    let navmesh_handle = generator.generate(settings);
    commands.spawn(DetailNavmeshGizmo::new(&navmesh_handle));
    commands.insert_resource(NavMeshHandle(navmesh_handle));
}*/

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
