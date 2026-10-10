use bevy::prelude::*;

mod applicator;
mod drill;
mod foam_gun;

pub use applicator::*;
pub use drill::*;
pub use foam_gun::*;

#[derive(Component, Default)]
#[relationship_target(relationship = AttachedToRover, linked_spawn)]
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

#[derive(EntityEvent)]
pub struct SwitchRoveerAttachmentEvent2 {
    pub entity: Entity,
    pub attachment: Entity,
}

#[derive(Component, Default)]
pub struct RoverAttachment;

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
