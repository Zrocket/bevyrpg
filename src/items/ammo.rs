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

#[derive(Debug, Clone, Component,  Reflect, Default)]
#[reflect(Component)]
pub struct AmmoPouch(pub i32);

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
            .register_type::<AmmoPouch>()
            .add_observer(ammo_event_observer);
    }
}


fn ammo_event_observer(
        trigger: Trigger<InteractEvent, Ammo>
) {
    let _player = trigger.event().actor;
    let _ammo = trigger.target();
}
