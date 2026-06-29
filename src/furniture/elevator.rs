use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

#[derive(Component)]
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

#[derive(Component)]
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
       app; 
    }
}

fn elevator_button_interaction_observer(
    _trigger: On<crate::InteractionEvent>,
    mut elevator_query: Query<(Entity, &mut Elevator)>,
) {
    if let Ok((entity, mut elevator)) = elevator_query.single_mut() {
        if elevator.current == elevator.max {
            elevator.current = elevator.min;
        } else {
            elevator.current += 1;
        }
    }
}
