use bevy::prelude::*;

use crate::{Interactable, InteractionEvent};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Consume;

pub struct ComsumePlugin;
impl Plugin for ComsumePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Consume>()
            .add_systems(Update, register_consume_items);
    }
}

fn register_consume_items(
    mut commands: Commands,
    mut unregistered_items_query: Query<Entity, (With<Consume>, Without<Interactable>)>,
) {
    for unregistered_item in unregistered_items_query.iter_mut() {
        commands.entity(unregistered_item).observe(consume_interaction_observer)
            .insert(Interactable);
    }
}

fn consume_interaction_observer(
    trigger: On<InteractionEvent, Consume>
) {
    let _actor = trigger.event().actor;
    let _consumeable = trigger.entity;
}
