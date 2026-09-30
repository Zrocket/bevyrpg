use std::time::Duration;

use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, math::Affine3A, mesh::VertexAttributeValues, prelude::*};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ElevatorCurve;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    crate::Interactable,
)]
#[component(on_add = on_elevator_down_button_add)]
pub struct ElevatorUpButton;

#[derive(Component)]
pub struct ElevatorInterpolation {
    pub duration: Duration,
    pub start_time: Duration,
    pub start_pos: Transform,
    pub desired_pos: Transform,
}

fn on_elevator_down_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(elevator_down_button_interaction_observer);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    crate::Interactable,
)]
#[component(on_add = on_elevator_up_button_add)]
pub struct ElevatorDownButton;

fn on_elevator_up_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(elevator_up_button_interaction_observer);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    crate::Interactable,
)]
#[component(on_add = on_elevator_button_add)]
pub struct ElevatorButton;

fn on_elevator_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(elevator_button_interaction_observer);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
)]
pub struct Elevator {
    min: usize,
    max: usize,
    current: usize,
}

pub struct ElevatorPlugin;
impl Plugin for ElevatorPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<Elevator>()
           .register_type::<ElevatorButton>()
           .register_type::<ElevatorCurve>()
           .add_systems(Update, drive_elevator_interpolation);
    }
}

fn elevator_button_interaction_observer(
    _trigger: On<crate::InteractionEvent>,
    mut commands: Commands,
    time: Res<Time>,
    meshes: Res<Assets<Mesh>>,
    mut elevator_query: Query<(Entity, &mut Elevator, &mut Transform, &GlobalTransform), (Without<ElevatorCurve>, Without<ElevatorInterpolation>)>,
    curve_mesh_query: Query<(&Mesh3d, &GlobalTransform), With<ElevatorCurve>>,
) {
    let Ok((entity, mut elevator, mut elevator_transform, elevator_global_transform)) = elevator_query.single_mut() else {
        error!("elevator_button_interaction_observer: Failed to query Entity, Elevator, Transforrm, GlobalTransform");
        return;
    };
    let Ok((curve_mesh3d, curve_global_transform)) = curve_mesh_query.single() else {
        error!("elevator_button_interaction_observer: Failed to query ElevatorCurve's Mesh3d and GlobalTransform");
        return;
    };
    let Some(mesh) = meshes.get(&curve_mesh3d.0) else {
        error!("elevator_button_interaction_observer: Failed to query Mesh for {:?}", curve_mesh3d.0);
        return;
    };
    let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
        error!("elevator_button_interaction_observer: Failed to get mesh attribute");
        return;
    };

    let scale = elevator_transform.scale;
    let rotation = elevator_transform.rotation;

    if let Some(_current_point) = positions.get(elevator.current) {
        elevator.current += 1;

        if let Some(next_point) = positions.get(elevator.current) {
            let point_vec = vec3(next_point[0], next_point[1], next_point[2]);

            commands.entity(entity)
                .insert(ElevatorInterpolation {
                    duration: time.elapsed() + Duration::new(1, 0),
                    start_time: time.elapsed(),
                    start_pos: *elevator_transform,
                    desired_pos: Transform { translation: point_vec, rotation, scale },
            });
        } else {
            elevator.current = 0;
            if let Some(next_point) = positions.get(elevator.current) {
                let point_vec = vec3(next_point[0], next_point[1], next_point[2]);

                commands.entity(entity)
                    .insert(ElevatorInterpolation {
                        duration: time.elapsed() + Duration::new(1, 0),
                        start_time: time.elapsed(),
                        start_pos: *elevator_transform,
                        desired_pos: Transform { translation: point_vec, rotation, scale },
                });
            }
        }
    }
}

fn drive_elevator_interpolation(
    mut commands: Commands,
    time: Res<Time>,
    mut elevator_query: Query<(Entity, &mut Transform, &ElevatorInterpolation), Without<ElevatorCurve>>,
) {
    for (
        elevator_entity,
        mut elevator_transform,
        elevator_interpolation
    ) in elevator_query.iter_mut() {
        if elevator_interpolation.duration <= time.elapsed() {
            commands.entity(elevator_entity).remove::<ElevatorInterpolation>();
            return;
        }

        let desired_transform = elevator_interpolation.desired_pos.translation;
        let desired_rotation = elevator_interpolation.desired_pos.rotation;
        let ease_function = EaseFunction::SmoothStep;
        let normalized_time = (time.elapsed() - elevator_interpolation.start_time).div_duration_f32(elevator_interpolation.duration - time.elapsed());

        if let Some(ease_normal) = ease_function.sample(normalized_time) {
            elevator_transform.translation = elevator_transform.translation.lerp(desired_transform, ease_normal);
        } else {
            commands.entity(elevator_entity).remove::<ElevatorInterpolation>();
            return;
        }
    }
}

fn elevator_up_button_interaction_observer(
    _trigger: On<crate::InteractionEvent>,
    time: Res<Time>,
    meshes: Res<Assets<Mesh>>,
    mut elevator_query: Query<(Entity, &mut Elevator, &mut Transform, &GlobalTransform), Without<ElevatorCurve>>,
    curve_mesh_query: Query<(&Mesh3d, &GlobalTransform), With<ElevatorCurve>>,
) {
    let Ok((entity, mut elevator, mut elevator_transform, elevator_global_transform)) = elevator_query.single_mut() else {
        error!("elevator_up_button_interaction_observer: Failed to query Entity, Elevator, Transform, GlobalTransform");
        return;
    };
    let Ok((curve_mesh3d, curve_global_transform)) = curve_mesh_query.single() else {
        error!("elevator_up_button_interaction_observer: Failed to query ElevatorCurve's Mesh3d and GlobalTransform");
        return;
    };
    let Some(mesh) = meshes.get(&curve_mesh3d.0) else {
        error!("elevator_up_button_interaction_observer: Failed to query Mesh for {:?}", curve_mesh3d.0);
        return;
    };
    let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
        error!("elevator_up_button_interaction_observer: Failed to get mesh attribute");
        return;
    };
    //if let Ok((entity, mut elevator, mut elevator_transform, elevator_global_transform)) = elevator_query.single_mut()
    //&& let Ok((curve_mesh3d, curve_global_transform)) = curve_mesh_query.single()
    //&& let Some(mesh) = meshes.get(&curve_mesh3d.0)
    //&& let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
    let ease_function = EaseFunction::SmoothStep;
    let scale = elevator_transform.scale;
    let rotation = elevator_transform.rotation;
    if let Some(_current_point) = positions.get(elevator.current) {
        elevator.current += 1;
        if let Some(next_point) = positions.get(elevator.current) {
            let point_vec = vec3(next_point[0], next_point[1], next_point[2]);

            *elevator_transform = Transform {
                translation:  point_vec,
                rotation,
                scale,
            };
        } else {
            elevator.current = 0;
            if let Some(next_point) = positions.get(elevator.current) {
                let point_vec = vec3(next_point[0], next_point[1], next_point[2]);
                *elevator_transform = elevator_global_transform.reparented_to(curve_global_transform);

                *elevator_transform = Transform {
                    translation:  point_vec,
                    rotation,
                    scale,
                };
            }
        }
    }
    //}
}

fn elevator_down_button_interaction_observer(
    _trigger: On<crate::InteractionEvent>,
    time: Res<Time>,
    meshes: Res<Assets<Mesh>>,
    mut elevator_query: Query<(Entity, &mut Elevator, &mut Transform, &GlobalTransform), Without<ElevatorCurve>>,
    curve_mesh_query: Query<(&Mesh3d, &GlobalTransform), With<ElevatorCurve>>,
) {
    let Ok((entity, mut elevator, mut elevator_transform, elevator_global_transform)) = elevator_query.single_mut() else {
        error!("elevator_down_button_interaction_observer: Failed to query Entity, Elevator, Transform, GlobalTransform");
        return;
    };
    let Ok((curve_mesh3d, curve_global_transform)) = curve_mesh_query.single() else {
        error!("elevator_down_button_interaction_observer: Failed to query ElevatorCurve's Mesh3d and GlobalTransform");
        return;
    };
    let Some(mesh) = meshes.get(&curve_mesh3d.0) else {
        error!("elevator_down_button_interaction_observer: Failed to query Mesh for {:?}", curve_mesh3d.0);
        return;
    };
    let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
        error!("elevator_down_button_interaction_observer: Failed to get mesh attribute");
        return;
    };
    //if let Ok((entity, mut elevator, mut elevator_transform, elevator_global_transform)) = elevator_query.single_mut()
    //&& let Ok((curve_mesh3d, curve_global_transform)) = curve_mesh_query.single()
    //&& let Some(mesh) = meshes.get(&curve_mesh3d.0)
    //&& let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
    let ease_function = EaseFunction::SmoothStep;
    let scale = elevator_transform.scale;
    let rotation = elevator_transform.rotation;
    if let Some(_current_point) = positions.get(elevator.current) {
        elevator.current -= 1;
        if let Some(next_point) = positions.get(elevator.current) {
            let point_vec = vec3(next_point[0], next_point[1], next_point[2]);

            *elevator_transform = Transform {
                translation:  point_vec,
                rotation,
                scale,
            };
        } else {
            elevator.current = 0;
            if let Some(next_point) = positions.get(elevator.current) {
                let point_vec = vec3(next_point[0], next_point[1], next_point[2]);
                *elevator_transform = elevator_global_transform.reparented_to(curve_global_transform);

                *elevator_transform = Transform {
                    translation:  point_vec,
                    rotation,
                    scale,
                };
            }
        }
    }
    //}
}
