use avian3d::prelude::{Collider, SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::{interact::Interaction, CollisionLayer, Player};

#[derive(Event)]
pub struct LadderEvent {
    actor: Entity,
    target: Entity,
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct LadderComponent;
impl Interaction for LadderComponent {
    fn interact(&self,commands: &mut Commands,entity:Entity,prop:Entity,) {
        println!("Ladder Interaction");
        commands.trigger_targets(LadderEvent {actor: entity, target: prop}, entity);
    }
}

pub struct LadderPlugin;
impl Plugin for LadderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LadderComponent>()
            .register_component_as::<dyn Interaction, LadderComponent>()
            .add_event::<LadderEvent>()
            .add_observer(ladder_event_observer);
    }
}

fn ladder_event_observer(
    trigger: Trigger<LadderEvent>,
    spatial_query: SpatialQuery,
    player_query: Query<&Collider, With<Player>>,
    ladder_query: Query<&GlobalTransform, With<LadderComponent>>,
) {
    trace!("OBSERVER: ladder_event_observer");
    if let Ok(ladder) = ladder_query.single()
        && let Ok(player) = player_query.single() {
            let temp = spatial_query.shape_intersections(
                &Collider::cuboid(4.0, 4.0, 4.0),
                ladder.translation(),
                ladder.rotation(),
                &SpatialQueryFilter::from_mask(CollisionLayer::Player)
            );
        println!("{:?}", temp);
    }
}
