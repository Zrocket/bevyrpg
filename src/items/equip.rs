use bevy::{ecs::{component::Component, entity::Entity, relationship}, prelude::Plugin};

pub enum EquipSlot {
    Arm,
    Leg,
    Hand,
    Feet,
    Body,
    Head,
    Finger,
    Toe,
}

#[derive(Component)]
pub struct Equiptable {
    pub slot: EquipSlot,
    pub defense: i32,
}

#[derive(Component)]
#[relationship_target(relationship = InEquiptment)]
pub struct Equiptment(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = Equiptment)]
pub struct InEquiptment(pub Entity);

pub struct EquipItemPlugin;
impl Plugin for EquipItemPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app; 
    }
}
