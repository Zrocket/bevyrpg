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
    min: i32,
    max: i32,
    current: i32,
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
    mut elevator_query: Query<(Entity, &mut Elevator)>,
    curve_mesh_query: Query<(&Mesh3d, &GlobalTransform), With<ElevatorCurve>>,
) {
    if let Ok((entity, mut elevator)) = elevator_query.single_mut()
    && let Ok((curve_mesh3d, curve_global_transform)) = curve_mesh_query.single()
    && let Some(mesh) = meshes.get(&curve_mesh3d.0)
    && let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        for &position in positions {
            let world_position = curve_global_transform.transform_point(Vec3::from(position));
            println!("{:?}", world_position);
        }

        /*if elevator.current == elevator.max {
            elevator.current = elevator.min;
        } else {
            elevator.current += 1;
        }*/
    }
}
