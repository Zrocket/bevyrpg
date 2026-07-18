use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, math::Affine3A, mesh::VertexAttributeValues, prelude::*};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ElevatorCurve;

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
           .register_type::<ElevatorCurve>();
    }
}

fn elevator_button_interaction_observer(
    _trigger: On<crate::InteractionEvent>,
    time: Res<Time>,
    meshes: Res<Assets<Mesh>>,
    mut elevator_query: Query<(Entity, &mut Elevator, &mut Transform, &GlobalTransform), Without<ElevatorCurve>>,
    curve_mesh_query: Query<(&Mesh3d, &GlobalTransform), With<ElevatorCurve>>,
) {
    if let Ok((entity, mut elevator, mut elevator_transform, elevator_global_transform)) = elevator_query.single_mut()
    && let Ok((curve_mesh3d, curve_global_transform)) = curve_mesh_query.single()
    && let Some(mesh) = meshes.get(&curve_mesh3d.0)
    && let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
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
    }
}
