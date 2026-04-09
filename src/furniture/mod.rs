use bevy::{ecs::query::QueryData, prelude::*};

mod bed;
mod cctv;
mod ceiling_light;
mod chair;
mod computer;
mod door;
mod ladder;
mod lever;

pub use bed::*;
pub use cctv::*;
pub use ceiling_light::*;
pub use chair::*;
pub use computer::*;
pub use door::*;
pub use ladder::*;
pub use lever::*;

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
            CeilingLightPlugin,
            ChairPlugin,
            ComputerPlugin,
            DoorPlugin,
            LadderPlugin,
        ));
    }
}
