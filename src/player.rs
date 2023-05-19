use bevy::{prelude::*, reflect};

use crate::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();
    }
}
