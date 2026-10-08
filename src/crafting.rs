//! # Crafting
//!
//! ## Overview
//! Provides systems and components for crafting
//!
//! ## Usage
//! ```no_run
//! # use bevy::prelude::*;
//! # use bevyrpg::crafting::CraftingPlugin;
//! App::new().add_plugins((DefaultPlugins, CraftingPlugin));
//! ```
//!
//! ## Related modules
//! - [`crate::items`]: provides data on all game items
//! - [`crate::ui::crafting_ui`]: displays [`UiCrafting`]

use std::collections::HashMap;

use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{LoadingStateAppExt, config::{ConfigureLoadingState, LoadingStateConfig}}};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::{BootStrap, Inventory, Player, RemoveFromInventoryEvent, container_interaction_observer, crafting_ui::display_crafting_ui, spawn_sample};

/// Sent when an item should be crafted by a CraftingStation
///
/// **Sent by:** on_craft_click
/// **Handled by:** [`craft_event_observer`].
#[derive(EntityEvent)]
pub struct CraftEvent {
    pub entity: Entity,
    pub id: String,
}

/// Starts crafting from a recipe if the player has the ingredients.
/// Blocks if an existing [`CraftTimer`] is present.
///
/// # Trigger
/// [`CraftEvent`],targeted at [`CraftingStation`].
fn craft_event_observer(
    trigger: On<CraftEvent>,
    mut commands: Commands,
    recipe_book: Res<RecipeBook>,
    inventory_query: Query<(Entity ,&Inventory), With<Player>>,
    tag_query: Query<&CraftTag>,
    crafting_station_query: Query<(Entity, Option<&CraftTimer>), With<CraftingStation>>,
) {
    println!("{}", trigger.id);
    let Ok((entity, inventory)) = inventory_query.single() else {
        error!("craft_event_observer: Failed to query Player Inventory");
        return;
    };
    let Some(recipe) = recipe_book.0.get(&trigger.id) else {
        error!("craft_event_observer: Failed to query RecipeBook");
        return;
    };
    let Ok((crafting_station, craft_timer)) = crafting_station_query.single() else {
        error!("craft_event_observer: Failed to query CraftingStation");
        return;
    };

    if craft_timer.is_none() {
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

/// Marks the entity as a Crafting Station.
#[derive(Component, Reflect, Clone, PartialEq, Eq, Hash, Debug)]
#[reflect(Component)]
#[require(
    crate::Interactable,
    ActiveRecipe(None),
    Name::new("Crafting Station"),
)]
#[component(on_add = on_crafting_station_add)]
pub struct CraftingStation;

/// [`CraftingStation`] on_add Hook
///
/// Adds necissary observers for [`CraftingStation`] functionality
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

/// The current active recipe selected by the [`CraftingStation`].
///
/// `None` indicates no recipie is active. Otherwise, the inner string holds the recipe's ID,
/// as keyed in [`RecipeBook`].
#[derive(Component)]
pub struct ActiveRecipe(pub Option<String>);

#[derive(Component, Reflect, Clone, PartialEq, Eq, Hash, Debug)]
#[reflect(Component)]
pub struct CraftTag(pub String);

/// The timer of a [`Recipe`] currently being crafted by a [`CraftingStation`]
#[derive(Component, Reflect, Clone, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub struct CraftTimer(pub Timer);

/// Updates an active [`CraftTimer`]
fn update_craft_timer(
    mut timer_query: Query<&mut CraftTimer>,
    time: Res<Time>,
) {
    if let Ok(mut timer) = timer_query.single_mut() {
        timer.0.tick(time.delta());
    }
}

/// A crafting recipe, loaded from `recipes.r.ron`.
#[derive(Asset, TypePath, Deserialize, Clone, Debug)]
pub struct Recipe {
    pub id: String,
    pub description: String,
    pub inputs: Vec<(String, u32)>,
    pub output_tag: String,
    pub output_name: String,
    pub craft_time: f32,
}

/// The array of all [`Recipe`]s loaded from `recipes.r.ron`
#[derive(Asset, TypePath, Deserialize, Clone, Debug)]
pub struct CraftingRecipes(Vec<Recipe>);

/// The top level handle for [`CraftingRecipes`]
#[derive(AssetCollection, Resource, TypePath, Clone, Debug)]
pub struct CraftingRecipesHandle{
    #[asset(path = "recipes.r.ron")]
    pub handle: Handle<CraftingRecipes>,
}

/// Global map of tags to [`Recipe`]s
#[derive(Resource)]
pub struct RecipeBook(
    /// Maps recipe tag [`String`] to [`Recipe`]
    pub HashMap<String, Recipe>
);
impl FromWorld for RecipeBook {
    fn from_world(world: &mut World) -> Self {
        let handle = world.resource::<CraftingRecipesHandle>();
        let recipes = world.resource::<Assets<CraftingRecipes>>();

        let list = recipes
            .get(&handle.handle)
            .expect("CraftingRecipes must be loaded before RecipeBook is initialized");

        RecipeBook(
            list.0
                .iter()
                .map(|recipe| (recipe.id.clone(), recipe.clone()))
                .collect()
        )

        /*let mut tmp = HashMap::<String, Recipe>::new();
        tmp.insert("fungacide".into(),
                    Recipe {
                        id: String::from("colloidal copper"),
                        description: String::from("a fungacide"),
                        inputs: vec![("test".into(), 1)],
                        output_tag: "colloidal_copper".into(),
                        output_name: "colloidal copper".into(),
                        craft_time: 100.,
                    },
            );

            Self(tmp)
        */
    }
}

/// Adds crafting systems, types, and resources
///
/// # Registers
/// - **Systems:** [`update_craft_timer`]
/// - **Types:** [`CraftingStation`]
/// - **Resources:** [`RecipeBook`] (in [`BootStrap::Loading`])
///
/// # Requires
/// - [`crate::StatesPlugin`] must be added first
pub struct CraftingPlugin;
impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_plugins(RonAssetPlugin::<CraftingRecipes>::new(&["r.ron"]))
           .configure_loading_state(
               LoadingStateConfig::new(BootStrap::Loading)
               .load_collection::<CraftingRecipesHandle>()
               .finally_init_resource::<RecipeBook>()
           )
           .register_type::<CraftingStation>()
           .add_systems(Update, update_craft_timer);
    }
}

/// Tallys up the [`CraftTag`]s of a given inventory
///
/// # Returns
/// [`std::collection::Hashmap<String, u32>`]
///
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

/// Determins if a given [`Recipe`] is craftable
///
/// # Returns
/// [`bool`] indicating if the given recipie is craftable
pub fn recipe_is_craftable(
    recipe: &Recipe,
    counts: &std::collections::HashMap<String, u32>,
) -> bool {
    recipe.inputs.iter().all(|(tag, need)| counts.get(tag).copied().unwrap_or(0) >= *need)
}
