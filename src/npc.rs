use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use bevy_bae::*;
use bevy_landmass::{Agent, Agent3dBundle, AgentSettings, AgentTarget3d, Archipelago3d, ArchipelagoRef3d};

use crate::{GameState, TnuaNpcController, Walk};

pub struct NpcPlugin;
impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(OnEnter(GameState::Gameplay), spawn_agent);
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
                AgentTarget3d::Point(Vec3::new(15.0, 1.5, 15.0)),
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                RigidBody::Dynamic,
                Collider::cuboid(1.0, 1.0, 1.0),
        ))
        .insert(TnuaNpcController)
        .insert(Walk::default())
        .insert(Name::new("Walking Cube"));
    }
}
