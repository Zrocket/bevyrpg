use bevy::prelude::*;

#[derive(Component, Default)]
pub struct RoverBattery {
    charge: i32,
}

pub struct RoverBatteryPlugin;
impl Plugin for RoverBatteryPlugin {
    fn build(&self, app: &mut App) {
       app; 
    }
}
