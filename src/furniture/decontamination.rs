use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_seedling::sample::SamplePlayer;

use crate::{CloseDoorEvent, DoorState, LockedState};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    crate::Interactable,
)]
#[component(on_add = on_decontamination_button_add)]
pub struct DecontaminationButton;

fn on_decontamination_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(decontamination_interaction_observer);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DecontaminationDoor;

pub struct DecontaminationPlugin;
impl Plugin for DecontaminationPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<DecontaminationButton>()
            .register_type::<DecontaminationDoor>();
    }
}

fn decontamination_interaction_observer(
    _trigger: On<crate::InteractionEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut door_query: Query<(Entity, &DoorState, &mut LockedState), With<DecontaminationDoor>>,
) {
    commands.spawn(SamplePlayer::new(asset_server.load("audio/compressed_gas_leak.ogg")));
    for (door, state, mut lock) in door_query.iter_mut() {
        if *state == DoorState::Open {
            commands.entity(door).trigger(|entity| CloseDoorEvent { entity });
        }
        if *lock == LockedState::Unlocked {
            *lock = LockedState::Locked;
        } else {
            *lock = LockedState::Unlocked;
        }
    }
}
