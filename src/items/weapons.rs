use bevy::prelude::*;

use crate::InteractEvent;

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
