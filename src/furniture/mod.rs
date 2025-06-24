use bevy::prelude::*;

mod chair;
mod computer;
mod door;
mod ladder;
mod lever;

pub use chair::*;
pub use computer::*;
pub use door::*;
pub use ladder::*;
pub use lever::*;

pub struct FurniturePlugin;
impl Plugin for FurniturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ChairPlugin,
            ComputerPlugin,
            DoorPlugin,
        ));
    }
}
