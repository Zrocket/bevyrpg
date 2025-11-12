use bevy::prelude::*;

use crate::{Interactable, InteractionEvent};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub enum ArmorType {
    #[default]
    None,
    Chest,
    Leg,
    Foot,
    Head,
    Arm,
    Hand,
    Face,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Armor {
    armor_type: ArmorType,
    defense: i32,
}

pub struct ArmorPlugin;

impl Plugin for ArmorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Armor>()
            .add_systems(Update, register_armor_items);
    }
}

fn register_armor_items(
    mut commands: Commands,
    mut unregistered_items_query: Query<Entity, (With<Armor>, Without<Interactable>)>,
) {
    for unregistered_item in unregistered_items_query.iter_mut() {
        commands.entity(unregistered_item).observe(armor_interaction_observer)
            .insert(Interactable);
    }
}

fn armor_interaction_observer(
        trigger: On<InteractionEvent, Armor>
) {
    let _player = trigger.event().actor;
    let _armor = trigger.entity;
}
