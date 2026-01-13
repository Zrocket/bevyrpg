use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{CharacterBundle, death_event_observer};

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_enemy_add)]
pub struct Enemy;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Enemy>();
    }
}

fn on_enemy_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .insert(CharacterBundle::default())
        .observe(death_event_observer);
}
