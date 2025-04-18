use bevy::prelude::*;

use crate::InteractEvent;

#[derive(Debug, Clone, Reflect, Default)]
pub enum AmmoType {
    #[default]
    None,
}

#[derive(Debug, Clone, Component,  Reflect, Default)]
#[reflect(Component)]
pub struct Ammo;

pub struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ammo>()
            .add_observer(ammo_observer_handler);
    }
}


fn ammo_observer_handler(
        trigger: Trigger<InteractEvent, Ammo>
) {
    let player = trigger.event().actor;
    let ammo = trigger.entity();
}
