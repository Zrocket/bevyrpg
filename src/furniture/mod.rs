use bevy::{ecs::query::QueryData, prelude::*};

mod bed;
mod chair;
mod computer;
mod door;
mod ladder;
mod lever;

pub use bed::*;
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
            ChairPlugin,
            ComputerPlugin,
            DoorPlugin,
            LadderPlugin,
        ));
    }
}
