use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Rover;

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}
