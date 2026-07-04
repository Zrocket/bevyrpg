use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, mesh::VertexAttributeValues, prelude::*};

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
    meshes: Res<Assets<Mesh>>,
    mut elevator_query: Query<(Entity, &mut Elevator, &mut Transform)>,
    curve_mesh_query: Query<(&Mesh3d, &GlobalTransform), With<ElevatorCurve>>,
) {
    if let Ok((entity, mut elevator, mut elevator_transform)) = elevator_query.single_mut()
    && let Ok((curve_mesh3d, curve_global_transform)) = curve_mesh_query.single()
    && let Some(mesh) = meshes.get(&curve_mesh3d.0)
    && let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        if let Some(_current_point) = positions.get(elevator.current) {
            elevator.current += 1;
            if let Some(next_point) = positions.get(elevator.current) {
                let world_position = curve_global_transform.transform_point(Vec3::from(*next_point));
                *elevator_transform = Transform {
                    translation:  world_position,
                    rotation: elevator_transform.rotation,
                    scale: elevator_transform.scale,
                }
            } else {
                elevator.current = 0;
                if let Some(next_point) = positions.get(elevator.current) {
                    let world_position = curve_global_transform.transform_point(Vec3::from(*next_point));
                    *elevator_transform = Transform {
                        translation:  world_position,
                        rotation: elevator_transform.rotation,
                        scale: elevator_transform.scale,
                    }
                }
            }
        }

        for &position in positions {
            let world_position = curve_global_transform.transform_point(Vec3::from(position));
            println!("{:?}", world_position);
        }
    }
}
