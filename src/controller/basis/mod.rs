use avian3d::math::AdjustPrecision;
use bevy::prelude::*;
use bevy_tnua::{TnuaBasis, TnuaProximitySensor, builtins::TnuaBuiltinWalkHeadroom, ghost_overrides::{TnuaGhostOverwrite, TnuaGhostOverwritesForBasis}, math::{Float, Vector3}, sensor_sets::{ProximitySensorPreparationHelper, TnuaSensors}};
use serde::{Deserialize, Serialize};

#[derive(Default)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct TnuaSwim {
    pub desired_motion: Vector3,
    pub desired_forward: Option<Dir3>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TnuaSwimConfig {
    pub speed: Float,
    pub float_height: Float,
    pub headroom: Option<TnuaBuiltinWalkHeadroom>,
    pub acceleration: Float,
    pub tilt_offset_angvel: Float,
    pub turning_angvel: Float,
}

#[derive(Default, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct TnuaSwimMemory {
    pub standing_offset: Vector3,
    effective_velocity: Vector3,
    vertical_velocity: Float,
    pub running_velocity: Vector3,
    extra_headroom: Float,
}

#[derive(Copy, Clone)]
pub struct TnuaSwimSensors<'a> {
    /// The main sensor of the floating character model.
    pub ground: &'a TnuaProximitySensor,

    /// An upward-facing sensor that checks for obstacles above the character.
    pub headroom: Option<&'a TnuaProximitySensor>,
}

impl <'a> TnuaSensors<'a> for TnuaSwimSensors<'a> {
    type Entities = TnuaSwimSensorsEntities;
    type GhostOverwrites = TnuaSwimSensorsGhostOverwrites;
}

#[derive(Component, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct TnuaSwimSensorsGhostOverwrites {
    /// The main sensor of the floating character model.
    pub ground: TnuaGhostOverwrite,
}

impl TnuaGhostOverwritesForBasis for TnuaSwimSensorsGhostOverwrites {
    type Entities = TnuaSwimSensorsEntities;
}

#[derive(Default)]
pub struct TnuaSwimSensorsEntities {
    /// The main sensor of the floating character model.
    pub ground: Option<Entity>,

    /// An upward-facing sensor that checks for
    pub headroom: Option<Entity>,
}

impl TnuaBasis for TnuaSwim {
    type Config = TnuaSwimConfig;

    type Memory = TnuaSwimMemory;

    type Sensors<'a> = TnuaSwimSensors<'a>;

    fn apply(
            &self,
            config: &Self::Config,
            memory: &mut Self::Memory,
            sensors: &Self::Sensors<'_>,
            ctx: bevy_tnua::TnuaBasisContext,
            motor: &mut bevy_tnua::TnuaMotor,
        ) {
        memory.extra_headroom = 0.0;

        let impulse_to_offset: Vector3;
        let slipping_vector: Option<Vector3>;

        if let Some(sensor_output) = &sensors.ground.output {
            memory.effective_velocity = ctx.tracker.velocity - sensor_output.entity_linvel;

            // Cross product of ground sensor and up direction
            let sideways_unnormalized = sensor_output
                .normal
                .cross(*ctx.up_direction)
                .adjust_precision();

            // if cross ground normal and up direction are the same
            if sideways_unnormalized == Vector3::ZERO {
            }
            impulse_to_offset = Vector3::ZERO;
        } else {
            memory.effective_velocity = ctx.tracker.velocity;
            impulse_to_offset = Vector3::ZERO;
        }
        memory.effective_velocity += impulse_to_offset;

        let velocity_on_plane = memory
            .effective_velocity
            .reject_from(ctx.up_direction.adjust_precision());

        let desired_velocity = self.desired_motion * config.speed;

        let desired_boost = desired_velocity - velocity_on_plane;
    }

    fn get_or_create_sensors<'a: 'b, 'b>(
            up_direction: Dir3,
            config: &'a Self::Config,
            memory: &Self::Memory,
            entities: &'a mut <Self::Sensors<'static> as bevy_tnua::sensor_sets::TnuaSensors<'static>>::Entities,
            proximity_sensors_query: &'b Query<(&bevy_tnua::TnuaProximitySensor, Has<bevy_tnua::TnuaGhostSensor>)>,
            controller_entity: Entity,
            commands: &mut Commands,
            has_ghost_overwrites: bool,
    ) -> Option<Self::Sensors<'b>> {
        let ground = ProximitySensorPreparationHelper {
            cast_direction: -up_direction,
            ghost_sensor: has_ghost_overwrites,
            ..Default::default()
        }
        .prepare_for(
            &mut entities.ground,
            proximity_sensors_query,
            controller_entity,
            commands
        );

        let headroom = if let Some(headroom) = config.headroom.as_ref() {
            ProximitySensorPreparationHelper {
                cast_direction: up_direction,
                cast_range: headroom.distance_to_collider_top
                    + headroom.sensor_extra_distance
                    + memory.extra_headroom,
                ..Default::default()
            }
            .prepare_for(
                &mut entities.headroom,
                proximity_sensors_query,
                controller_entity,
                commands
            )
        } else {
            ProximitySensorPreparationHelper::ensure_not_existing(
                &mut entities.headroom,
                proximity_sensors_query,
                commands
            )
        };

        Some(Self::Sensors {
            ground: ground?,
            headroom,
        })
    }

    fn ghost_sensor_overwrites<'a>(
            ghost_overwrites: &'a mut <Self::Sensors<'static> as bevy_tnua::sensor_sets::TnuaSensors<'static>>::GhostOverwrites,
            entities: &<Self::Sensors<'static> as bevy_tnua::sensor_sets::TnuaSensors<'static>>::Entities,
        ) -> impl Iterator<Item = (&'a mut bevy_tnua::ghost_overrides::TnuaGhostOverwrite, Entity)> {
            [(&mut ghost_overwrites.ground, entities.ground)]
                .into_iter()
                .flat_map(|(o, e)| Some((o, e?)))
    }
}
