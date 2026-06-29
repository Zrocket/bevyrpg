use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SampleItem {
    pub info: Vec<String>,
    pub analyzed: i32,
    pub botched: bool,
}

pub struct SampleItemPlugin;
impl Plugin for SampleItemPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<SampleItem>(); 
    }
}
