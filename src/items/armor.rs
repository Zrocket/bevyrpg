use avian_pickup::input::AvianPickupInput;
use bevy::{ecs::system::QueryLens, prelude::*};

use crate::{interact::Interaction, InteractEvent, Interactable};

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

impl Interaction for Armor {
    fn interact(
        &self,
        actor: &Entity,
        query: QueryLens<&Interactable>,
    ) -> Option<AvianPickupInput>
    {
        None
    }
}

pub struct ArmorPlugin;

impl Plugin for ArmorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Armor>()
            .add_observer(armor_observer_handler);
    }
}

fn armor_observer_handler(
        trigger: Trigger<InteractEvent, Armor>
) {
    let player = trigger.event().actor;
    let armor = trigger.entity();
}
