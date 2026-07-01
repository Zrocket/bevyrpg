use bevy::{ecs::query::QueryData, prelude::*};

mod analyzer;
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
mod elevator;

pub use analyzer::*;
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

use crate::furniture::elevator::ElevatorPlugin;

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
            AnalyzerPlugin,
            CctvPlugin,
            CeilingLightPlugin,
            ChairPlugin,
            ComputerPlugin,
            DoorPlugin,
            GaragePlugin,
            LadderPlugin,
            ElevatorPlugin,
        ));
    }
}
