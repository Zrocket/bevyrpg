use bevy::prelude::*;

use crate::{Interactable, InteractionEvent};

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
        app.register_type::<Weapon>();
    }
}

fn register_weapon_items(
    mut commands: Commands,
    mut unregistered_items_query: Query<Entity, (With<Weapon>, Without<Interactable>)>,
) {
    for unregistered_item in unregistered_items_query.iter_mut() {
        commands.entity(unregistered_item).observe(weapon_interaction_observer)
            .insert(Interactable);
    }
}

fn weapon_interaction_observer(
    trigger: On<InteractionEvent, Weapon>
) {
    let _actor = trigger.event().actor;
    let _weapon = trigger.entity;
}
