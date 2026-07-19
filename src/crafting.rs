use std::collections::HashMap;

use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Inventory, Player, RemoveFromInventoryEvent, container_interaction_observer, crafting_ui::display_crafting_ui};

#[derive(EntityEvent)]
pub struct CraftEvent {
    pub entity: Entity,
    pub id: String,
}

fn craft_event_observer(
    trigger: On<CraftEvent>,
    mut commands: Commands,
    recipe_book: Res<RecipeBook>,
    inventory_query: Query<(Entity ,&Inventory), With<Player>>,
    tag_query: Query<&CraftTag>,
    crafting_station_query: Query<Entity, With<CraftingStation>>,
) {
    if let Ok((entity, inventory)) = inventory_query.single()
    && let Some(recipe) = recipe_book.0.get(&trigger.id)
    && let Ok(crafting_station) = crafting_station_query.single() {
        let tags = tally_tags(inventory, &tag_query);
        if recipe_is_craftable(recipe, &tags) {
            for (id, num) in &recipe.inputs {
                let mut current = 0;
                for item in inventory.iter() {
                    if let Ok(tag) = tag_query.get(item)
                    && tag.0 == *id {
                        current += 1;
                        commands.entity(entity).trigger(|entity| RemoveFromInventoryEvent { entity, item});
                    }
                    if current == *num {
                        break;
                    }
                }
            }
                commands.entity(crafting_station).insert(CraftTimer(Timer::from_seconds(recipe.craft_time, TimerMode::Once)));
                println!("NEW TIMER");
        }
    }
}

#[derive(Component, Reflect, Clone, PartialEq, Eq, Hash, Debug)]
#[reflect(Component)]
#[require(
    crate::Interactable,
    Name::new("Crafting Station"),
)]
#[component(on_add = on_crafting_station_add)]
pub struct CraftingStation;

fn on_crafting_station_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(container_interaction_observer)
        .observe(display_crafting_ui)
        .observe(craft_event_observer);
}

#[derive(Component, Reflect, Clone, PartialEq, Eq, Hash, Debug)]
#[reflect(Component)]
pub struct CraftTag(pub String);

#[derive(Component, Reflect, Clone, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub struct CraftTimer(pub Timer);

fn update_craft_timer(
    mut timer_query: Query<&mut CraftTimer>,
    time: Res<Time>,
) {
    if let Ok(mut timer) = timer_query.single_mut() {
        timer.0.tick(time.delta());
    }
}

#[derive(Clone, Debug, Reflect)]
pub struct Recipe {
    pub id: String,
    pub description: String,
    pub inputs: Vec<(String, u32)>,
    pub output_tag: String,
    pub output_name: String,
    pub craft_time: f32,
}

#[derive(Resource)]
pub struct RecipeBook(pub HashMap<String, Recipe>);
impl FromWorld for RecipeBook {
    fn from_world(world: &mut World) -> Self {
        let mut tmp = HashMap::<String, Recipe>::new();
        tmp.insert("fungacide".into(),
                    Recipe {
                        id: String::from("fungacide"),
                        description: String::from("a fungacide"),
                        inputs: vec![("test".into(), 1)],
                        output_tag: "fungacide".into(),
                        output_name: "fungacide".into(),
                        craft_time: 100.,
                    },
            );

            Self(tmp)
    }
}

pub struct CraftingPlugin;
impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
       app
           .init_resource::<RecipeBook>()
           .register_type::<CraftingStation>()
           .add_systems(Update, update_craft_timer);
    }
}

pub fn tally_tags(
    inventory: &Inventory,
    tag_query: &Query<&CraftTag>,
) -> std::collections::HashMap<String, u32> {
    let mut counts = std::collections::HashMap::new();
    for item in inventory.iter() {
        if let Ok(tag) = tag_query.get(item) {
            *counts.entry(tag.0.clone()).or_insert(0) += 1;
        }
    }
    counts
}

pub fn recipe_is_craftable(
    recipe: &Recipe,
    counts: &std::collections::HashMap<String, u32>,
) -> bool {
    recipe.inputs.iter().all(|(tag, need)| counts.get(tag).copied().unwrap_or(0) >= *need)
}
