use avian_pickup::input::AvianPickupInput;
use bevy::prelude::*;

use crate::{interact::Interaction, InteractEvent};

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
        commands: &mut Commands,
        _actor: &Entity,
        prop: &Entity,
    ) -> Option<AvianPickupInput>
    {
        commands.entity(*prop).despawn();
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
    let _actor = trigger.event().actor;
    let _weapon = trigger.entity();
}
