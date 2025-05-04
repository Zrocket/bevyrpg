use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Rover;

#[derive(Event)]
pub struct RoverCommandEvent {}

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, _app: &mut App) {
    }
}
