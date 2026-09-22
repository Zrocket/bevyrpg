use bevy::prelude::*;

mod applicator;
mod drill;
mod foam_gun;

pub use applicator::*;
pub use drill::*;
pub use foam_gun::*;

#[derive(Component, Default)]
#[relationship_target(relationship = AttachedToRover)]
pub struct RoverAttachments(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = RoverAttachments)]
pub struct AttachedToRover(pub Entity);

#[derive(EntityEvent)]
pub struct UseRoverAttachmentEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct SwitchRoveerAttachmentEvent {
    pub entity: Entity,
    pub attachment: Attachment,
}

pub enum Attachment {
    Applicator,
    Drill,
    FoamGun,
}

pub struct RoverAttachmenntPlugin;
impl Plugin for RoverAttachmenntPlugin {
    fn build(&self, app: &mut App) {
       app;
    }
}
