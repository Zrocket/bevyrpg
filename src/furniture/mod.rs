use bevy::prelude::*;

mod chair;
mod computer;
mod door;

pub use chair::*;
pub use computer::*;
pub use door::*;

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
