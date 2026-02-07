use avian3d::prelude::{Collider, RayCaster, RayHits, SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_bae::{prelude::OperatorInput, task::OperatorStatus, *};
use bevy_landmass::{Agent, Agent3dBundle, AgentSettings, AgentTarget3d, Archipelago3d, ArchipelagoRef3d};
use rand::Rng;

use crate::{Player, TnuaNpcController, enemy::Enemy};

#[derive(Component)]
pub struct IdleTimer(pub Timer);

pub struct NpcPlugin;
impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_plugins(BaePlugin::default());
           //.add_systems(OnEnter(GameState::Gameplay), spawn_agent);
    }
}

fn spawn_agent(
    archipelago: Query<Entity, With<Archipelago3d>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(archipelago) = archipelago.single() {
        // Create an agent that will find a path as soon as the nav mesh is generated.
        commands.spawn((
                Agent3dBundle {
                    agent: Agent::default(),
                    archipelago_ref: ArchipelagoRef3d::new(archipelago),
                    settings: AgentSettings {
                        desired_speed: 5.0,
                        max_speed: 10.0,
                        radius: 0.5
                    },
                },
                Transform::from_xyz(-5.0, 0.5, -15.0),
                AgentTarget3d::Point(Vec3::new(15.0, 1.75, 15.0)),
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Collider::cuboid(1.0, 1.0, 1.0),
        ))
        .insert(TnuaNpcController)
        .insert(Name::new("Walking Cube"));
    }
}

pub fn run_from_player(
    In(input): In<OperatorInput>,
    mut commands: Commands,
    spatial_query: SpatialQuery,
    player_query: Query<(Entity, &Transform), With<Player>>,
    transform_query: Query<&Transform, Without<Player>>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single()
    && let Ok(agent_transform) = transform_query.get(input.entity)
    && let Ok(direction) = Dir3::new((player_transform.translation - agent_transform.translation).normalize_or_zero()) {
        spatial_query.cast_ray(agent_transform.translation, direction, 100., true, &SpatialQueryFilter::default());
    }
}

pub fn wander(
    In(input): In<OperatorInput>,
    mut agent_target_query: Query<&mut AgentTarget3d>,
) -> OperatorStatus {
    if let Ok(mut agent_target) = agent_target_query.get_mut(input.entity) {
        let mut rng = rand::rng();
        *agent_target = AgentTarget3d::Point(Vec3::new(rng.random_range(-15.0..15.0), 1.75, rng.random_range(-15.0..15.0)));
        return OperatorStatus::Success;
    }
    OperatorStatus::Failure
}

pub fn idle(
    In(input): In<OperatorInput>,
    mut idle_timer_query: Query<&mut IdleTimer>,
    time: Res<Time>,
) -> OperatorStatus {
    if let Ok(mut idle_timer) = idle_timer_query.get_mut(input.entity) {
        idle_timer.0.tick(time.delta());
        if idle_timer.0.just_finished() {
            idle_timer.0.reset();
            return OperatorStatus::Success;
        } else {
            return OperatorStatus::Ongoing;
        }
    }
    OperatorStatus::Failure
}
