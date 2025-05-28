use bevy::prelude::*;

use crate::{interact::Interaction, InteractEvent};

#[derive(Debug, Clone, Reflect, Default)]
pub enum AmmoType {
    #[default]
    None,
}

#[derive(Debug, Clone, Component,  Reflect, Default)]
#[reflect(Component)]
pub struct Ammo;

impl Interaction for Ammo {
    fn interact(
        &self,
        commands: &mut Commands,
        _actor: Entity,
        prop: Entity,
    ) {
        commands.entity(prop).despawn();
    }
}

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
    let _player = trigger.event().actor;
    let _ammo = trigger.target();
}
