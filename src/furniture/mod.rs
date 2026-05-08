use bevy::{ecs::query::QueryData, prelude::*};

mod bed;
mod cctv;
mod ceiling_light;
mod chair;
mod decontamination;
mod computer;
mod door;
mod garage;
mod ladder;
mod lever;
mod vending;

pub use bed::*;
pub use cctv::*;
pub use ceiling_light::*;
pub use chair::*;
pub use decontamination::*;
pub use computer::*;
pub use door::*;
pub use garage::*;
pub use ladder::*;
pub use lever::*;
pub use vending::*;

#[derive(QueryData)]
pub struct ObstacleQueryHelper {
    pub climbable: Has<Climbable>,
}

#[derive(Component, Default)]
pub struct Climbable;

pub struct FurniturePlugin;
impl Plugin for FurniturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CctvPlugin,
            CeilingLightPlugin,
            ChairPlugin,
            ComputerPlugin,
            DoorPlugin,
            GaragePlugin,
            LadderPlugin,
        ));
    }
}
