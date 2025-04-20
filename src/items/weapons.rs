use avian_pickup::input::AvianPickupInput;
use bevy::{ecs::system::QueryLens, prelude::*};

use crate::{interact::Interaction, InteractEvent, Interactable};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub enum WeaponType {
    #[default]
    None,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Weapon {
    weapon_type: WeaponType,
}

impl Interaction for Weapon {
    fn interact(
        &self,
        actor: &Entity,
        query: QueryLens<&Interactable>,
    ) -> Option<AvianPickupInput>
    {
        None
    }
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Weapon>()
            .add_observer(interact_observer_handler);
    }
}

fn interact_observer_handler(
    trigger: Trigger<InteractEvent, Weapon>
) {
    let actor = trigger.event().actor;
    let weapon = trigger.entity();
}
