use avian3d::prelude::Collider;
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_landmass::{Agent, Agent3dBundle, AgentSettings, AgentTarget3d, Archipelago3d, ArchipelagoRef3d};

use crate::{CharacterBundle, death_event_observer, TnuaEnemyController, GameState};

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
#[require(
    TnuaEnemyController,
)]
#[component(on_add = on_enemy_add)]
pub struct Enemy;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Enemy>()
            .add_systems(OnEnter(GameState::Gameplay), spawn_enemy_agent);
    }
}

fn on_enemy_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .insert(CharacterBundle::default())
        .observe(death_event_observer);
}

fn spawn_enemy_agent(
    archipelago: Query<Entity, With<Archipelago3d>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(archipelago) = archipelago.single() {
        commands.spawn((
                Agent3dBundle {
                    agent: Agent::default(),
                    archipelago_ref: ArchipelagoRef3d::new(archipelago),
                    settings: AgentSettings {
                        desired_speed: 5.0,
                        max_speed: 10.0,
                        radius: 0.5,
                    },
                },
                Transform::from_xyz(-5.0, 0.5, -15.0),
                AgentTarget3d::Point(Vec3::new(15.0, 1.75, 15.0)),
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Collider::cuboid(1.0, 1.0, 1.0),
        ))
        .insert(Name::new("Enemy Cube"))
        .insert(Enemy);
    }
}
