mod math_trait_ext;
mod pipe;
mod util_systems;

use bevy::ecs::component::Component;
//pub use criteria::*;
pub use math_trait_ext::*;
pub use pipe::*;
pub use util_systems::*;

#[derive(Component)]
pub struct Shelf<T>(pub Box<T>);
