use avian3d::prelude::{Collider, Friction, GravityScale, LockedAxes, RigidBody, SpatialQueryFilter};
use avian_pickup::actor::{AvianPickupActor, AvianPickupActorHoldConfig};
use bevy::{prelude::*, render::RenderPlugin};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_tnua::{control_helpers::TnuaSimpleAirActionsCounter, prelude::TnuaController};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;

use crate::{CameraConfig, CharacterBundle, CollisionLayer, Experience, FloatHeight, Health, Inventory, Mana, MaxHealth, MaxMana, PlayerController, PlayerControllerInput, RayHit, RenderPlayer, Walk};

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, States)]
pub enum PlayerState {
    #[default]
    Grounded,
    Ladder,
    UnGrounded,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerCamera;

pub struct GamePlayerPlugin;
impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        info!("GamePlayerPlugin build");
        app.register_type::<Player>()
            .register_type::<PlayerCamera>();
    }
}

fn player_initialize(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let logical_entity = commands.spawn((
            Player,
            PlayerController::default(),
            PlayerControllerInput::default(),
            CharacterBundle {
                mana: Mana(100),
                max_mana: MaxMana(100),
                health: Health(100),
                max_health: MaxHealth(100),
                experience: Experience(100),
                ..default()
            },
            Inventory { ..default() },
            Collider::capsule(0.1, 0.5),
            Friction {
                combine_rule: avian3d::prelude::CoefficientCombine::Min,
                ..default()
            },
            TnuaController::default(),
            TnuaAvian3dSensorShape(Collider::capsule(0.1, 0.1)),
            FloatHeight(1.5),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            GravityScale(1.0),
            Transform::from_xyz(0.0, 5.0, 0.0),
            CameraConfig {
                height_offset: 0.0,
            },
    ))
        .insert((
                Walk::default(),
                TnuaSimpleAirActionsCounter::default(),
                AvianPickupActor {
                    prop_filter: SpatialQueryFilter::from_mask(CollisionLayer::Prop),
                    actor_filter: SpatialQueryFilter::from_mask(CollisionLayer::Player),
                    obstacle_filter: SpatialQueryFilter::from_mask(CollisionLayer::Default),
                    hold: AvianPickupActorHoldConfig {
                        pitch_range: -40.0_f32.to_radians()..=75.0_f32.to_radians(),
                        distance_to_allow_holding: 100.0,
                        ..default()
                    },
                    ..default()
                },
                RayHit(Entity::PLACEHOLDER),
        ))
        .id();

    commands.spawn((
            Camera {
                hdr: true,
                clear_color: ClearColorConfig::Custom(Srgba::rgb(0.0, 0.0, 0.0).into()),
                ..default()
            },
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection { fov: std::f32::consts::PI / 2.0,
                ..default()
            }),
            Transform {
                translation: Vec3 { y: 2., ..default() },
                ..default()
            },
            RenderPlayer { logical_entity },
            PlayerCamera,
            AtmosphereCamera::default(),
    ));
}
