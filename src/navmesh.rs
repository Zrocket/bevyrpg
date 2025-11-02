use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use vleue_navigator::VleueNavigatorPlugin;
use super::utils::{F32Ext, Vec3Ext};
use bevy_rerecast::{Mesh3dBackendPlugin, prelude::*};

use crate::{GameState, Player, error_pipe};

#[derive(Resource)]
struct NavMeshHandle(Handle<Navmesh>);

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

pub struct NavMeshPlugin;
impl Plugin for NavMeshPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(NavmeshPlugins::default())
            .add_plugins(Mesh3dBackendPlugin::default())
            .register_type::<Walk>()
            .register_type::<FloatHeight>()
            .register_type::<DesiredPosition>()
            .add_plugins(TnuaAvian3dPlugin::new(FixedUpdate))
            .add_plugins(TnuaControllerPlugin::new(FixedUpdate))
            .add_plugins(VleueNavigatorPlugin)
            .add_systems(OnEnter(GameState::Loading), generate_navmesh);
            //.add_systems(
            //Update,
            //(
                //navmesh_pathfinding.pipe(error_pipe),
         //       apply_walking,
            //),
        //);
    }
}

fn generate_navmesh(
    mut generator: NavmeshGenerator,
    mut commands: Commands,
) {
    let agent_radius = 0.6;
    let agent_height = 1.8;
    let settings = NavmeshSettings::from_agent_3d(agent_radius, agent_height);
    let navmesh_handle = generator.generate(settings);
    commands.insert_resource(NavMeshHandle(navmesh_handle));
}

/*fn navmesh_pathfinding(
    nav_mesh: Res<NavMeshHandle>,
    nav: Res<Assets<Navmesh>>,
    asset_server: AssetServer,
    mut query: Query<(&Transform, &DesiredPosition, &mut Walk)>,
) -> anyhow::Result<()> {
    trace!("SYSTEM: navmesh_pathfinding");
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
