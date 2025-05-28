use bevy::prelude::*;

use crate::{interact::Interaction, InteractEvent};

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
        commands: &mut Commands,
        _actor: Entity,
        prop: Entity,
    ) {
        commands.entity(prop).despawn();
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
    let _player = trigger.event().actor;
    let _armor = trigger.target();
}
