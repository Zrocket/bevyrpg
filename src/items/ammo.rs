use bevy::prelude::*;

use crate::{Interactable, InteractionEvent};

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

pub struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ammo>()
            .register_type::<AmmoPouch>()
            .add_systems(Update, register_ammo_items);
    }
}

fn register_ammo_items(
    mut commands: Commands,
    mut unregistered_items_query: Query<Entity, (With<Ammo>, Without<Interactable>)>,
) {
    for unregistered_item in unregistered_items_query.iter_mut() {
        commands.entity(unregistered_item).observe(ammo_interaction_observer)
            .insert(Interactable);
    }
}

fn ammo_interaction_observer(
        trigger: On<InteractionEvent, Ammo>
) {
    let _player = trigger.event().actor;
    let _ammo = trigger.entity;
}
