use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    crate::Drillable("Fungus".into()),
)]
pub struct Fungus;

pub struct FungusPlugin;
impl Plugin for FungusPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<Fungus>();
    }
}
