use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{DrinkEvent, EatEvent, InteractionEvent, misc_inspection_observer, misc_pickup_observer};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    crate::Interactable,
)]
#[component(on_add = on_food_add)]
pub struct Food;

fn on_food_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(misc_pickup_observer)
        .observe(misc_inspection_observer)
        .observe(food_interaction_observer);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    crate::Interactable,
)]
#[component(on_add = on_drink_add)]
pub struct Drink;

fn on_drink_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(misc_pickup_observer)
        .observe(misc_inspection_observer)
        .observe(drink_interaction_observer);
}

pub struct ConsumableItemPlugin;
impl Plugin for ConsumableItemPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<Food>()
           .register_type::<Drink>();
    }
}

fn food_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
) {
    commands.entity(trigger.actor).trigger(|entity| EatEvent { entity });
    commands.entity(trigger.entity).despawn();
}

fn drink_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
) {
    commands.entity(trigger.actor).trigger(|entity| DrinkEvent { entity });
    commands.entity(trigger.entity).despawn();
}
