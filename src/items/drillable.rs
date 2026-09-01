use std::collections::HashMap;

use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*, state::commands};

use crate::{AddToInventoryEvent, ItemDatabase, ItemDetails, ItemId, Rover, SampleItem};

/*#[derive(Resource)]
pub struct DrillSampleTable(pub HashMap<String, fn(&mut Commands) -> Entity>);
impl FromWorld for DrillSampleTable {
    fn from_world(world: &mut World) -> Self {
        let tmp = HashMap::new();
    }
}*/

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[component(on_add = on_drillable_add)]
pub struct Drillable(pub String);

fn on_drillable_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(drill_event_observer);
}

#[derive(EntityEvent)]
pub struct DrillEvent{ pub entity: Entity }

pub struct DrillableItemPlugin;
impl Plugin for DrillableItemPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<Drillable>();
    }
}

fn drill_event_observer(
    trigger: On<DrillEvent>,
    mut commands: Commands,
    item_database: Res<ItemDatabase>,
    drillable_query: Query<&Drillable>,
    rover_query: Query<Entity, With<Rover>>,
) {
    println!("DRILLDRILLDRILL");
    if let Ok(drillable) = drillable_query.get(trigger.entity)
    && let Some(item_details) = item_database.0.get(&drillable.0)
    && let Ok(rover) = rover_query.single() {
        let item = commands.spawn((
                SampleItem {
                    analyzed: false,
                    botched: false,
                },
                ItemDetails {
                    name: item_details.name.clone(),
                    description: super::Description("SAMPLE".to_string()),
                    weight: super::Weight(5),
                },
                ItemId(item_details.id.clone()),
            )).id();
        //let item = spawn_sample(&mut commands);
        commands.entity(rover).trigger(|entity| AddToInventoryEvent { entity, item });
    }
}

pub fn spawn_sample(commands: &mut Commands) -> Entity {
    commands.spawn((SampleItem {
            analyzed: false,
            botched: false,
        },
        ItemDetails {
            name: "SAMPLE".to_string(),
            description: super::Description("SAMPLE".to_string()),
            weight: super::Weight(5),
        }
    )).id()
}
