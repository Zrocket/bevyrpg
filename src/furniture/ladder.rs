use avian3d::prelude::{OnCollisionEnd, OnCollisionStart};
use bevy::prelude::*;

use crate::{Player, PlayerState};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct LadderComponent;

pub struct LadderPlugin;
impl Plugin for LadderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LadderComponent>();
    }
}

pub fn ladder_collision_observer(
    trigger: Trigger<OnCollisionStart>,
    mut player_query: Query<&mut PlayerState, With<Player>>,
) {
    if player_query.contains(trigger.collider) && let Ok(mut player) = player_query.single_mut() {
        *player = PlayerState::Ladder(trigger.body.unwrap());
    }
}

pub fn ladder_decollision_observer(
    trigger: Trigger<OnCollisionEnd>,
    mut player_query: Query<&mut PlayerState, With<Player>>,
) {
    if player_query.contains(trigger.collider) && let Ok(mut player) = player_query.single_mut() {
        *player = PlayerState::Grounded;
    }
}
