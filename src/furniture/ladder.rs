use avian3d::prelude::{CollidingEntities, CollisionEnd, CollisionEventsEnabled, CollisionStart};
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Climbable, Player, PlayerState, level::BlenderTranslationComplete};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_ladder_add)]
#[require(
    CollidingEntities::default(),
    CollisionEventsEnabled,
    BlenderTranslationComplete,
    Climbable,
)]
pub struct LadderComponent;

pub struct LadderPlugin;
impl Plugin for LadderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LadderComponent>();
    }
}

fn on_ladder_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .queue_silenced(|mut entity: EntityWorldMut| {
            entity
                .observe(ladder_collision_observer)
                .observe(ladder_decollision_observer);
        });
}

pub fn ladder_collision_observer(
    trigger: On<CollisionStart>,
    mut player_query: Query<&mut PlayerState, With<Player>>,
) {
    trace!("OBSERVER: ladder_collision_observer");
    if player_query.contains(trigger.event().collider2) && let Ok(mut player) = player_query.single_mut() {
        *player = PlayerState::Ladder(trigger.event().body1.unwrap());
    }
}

pub fn ladder_decollision_observer(
    trigger: On<CollisionEnd>,
    mut player_query: Query<&mut PlayerState, With<Player>>,
) {
    trace!("OBSERVER: ladder_decollision_observer");
    if player_query.contains(trigger.event().collider2) && let Ok(mut player) = player_query.single_mut() {
        *player = PlayerState::Grounded;
    }
}
