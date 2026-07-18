use std::collections::HashMap;

use bevy::{color::palettes::css::{BLUE, DARK_KHAKI, DARK_SLATE_GRAY, DARK_TURQUOISE, DARK_VIOLET, GRAY, GREEN, PINK, YELLOW}, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{CraftEvent, CraftTag, CraftingStation, DisplayInventoryEvent, Inventory, ItemDetails, Player, RecipeBook, UiState, analyzer_ui::ProgressTimer, recipe_is_craftable, tally_tags, widgets::{floating_windows::floating_window_root, progress_bar::ProgressBar}};

#[derive(Component, Reflect)]
#[require(
    Node {
        ..default()
    },
    Text("CRAFT".into()),
)]
#[component(on_add = on_ui_craft_button_add)]
pub struct UiCraftButton(bool);

fn on_ui_craft_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let enabled = world.get::<UiCraftButton>(context.entity).unwrap().0.clone();

    if enabled {
        world.commands()
            .entity(context.entity)
            .observe(on_craft_click);
    }
}

#[derive(Component, Reflect)]
#[require(
    Node {
        padding: UiRect::axes(px(0.), px(10.)),
        align_self: AlignSelf::Center,
        width: percent(90.),
        ..default()
    },
    Text("PROGRESS".into()),
    BackgroundColor::from(DARK_TURQUOISE),
)]
pub struct UiCraftingProgressBar;

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        flex_direction: FlexDirection::Row,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor::from(DARK_VIOLET),
)]
pub struct UiCraftingRoot;

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        flex_direction: FlexDirection::Row,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor::from(DARK_VIOLET),
)]
pub struct UiCrafting;

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        flex_direction: FlexDirection::Column,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    Text("Recipes".into()),
    BackgroundColor::from(DARK_SLATE_GRAY),
)]
pub struct UiRecipes;

#[derive(Component, Reflect)]
#[require(
    Node {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor::from(PINK),
)]
#[component(on_add = on_ui_recipe_add)]
pub struct UiRecipe(pub String);

fn on_ui_recipe_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(on_recipe_click);
}

#[derive(Component)]
#[require(
    Node {
        width: Val::Px(100.),
        height: Val::Px(100.),
        ..default()
    },
    BackgroundColor::from(YELLOW),
)]
pub struct UiRecipeIcon;

#[derive(Component)]
#[require(
    Node {
        justify_content: JustifyContent::Center,
        flex_grow: 1.,
        ..default()
    },
    BackgroundColor::from(GREEN),
)]
#[component(on_add = on_ui_recipe_text_add)]
pub struct UiRecipeText(String);

fn on_ui_recipe_text_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let text = world.get::<UiRecipeText>(context.entity).unwrap().0.clone();

    world.commands()
        .entity(context.entity)
        .insert(Text(text));
}

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        flex_direction: FlexDirection::Column,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor::from(DARK_VIOLET),
)]
pub struct UiActiveRecipe(Option<String>);

pub struct CraftingUiPlugin;
impl Plugin for CraftingUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(Update, sync_active_recipe);
    }
}

pub fn display_crafting_ui(
    trigger: On<DisplayInventoryEvent>,
    recipe_book: Res<RecipeBook>,
    mut commands: Commands,
    name_query: Query<&Name>,
    item_query: Query<&ItemDetails>,
    inventory: Query<&Inventory>,
    menu_state: Res<State<UiState>>,
    mut menu_state_setter: ResMut<NextState<UiState>>,
) {
    let Ok(name) = name_query.get(trigger.entity) else {
        return;
    };

    let mut item_vec = vec![];

    if let Ok(inventory_handle) = inventory.get(trigger.entity) {
        for item in inventory_handle.iter() {
            if let Ok(item_name) = item_query.get(item) {
                trace!("Pushing item: {:?}, item_name: {:?}, to item_vec", item, item_name.name);
                item_vec.push((item_name.clone(), item.clone(), trigger.entity.clone()));
            }
        }
    }

    let mut recipes = vec![];

    for (recipe_id, recipe) in recipe_book.0.iter() {
        recipes.push(recipe_id.clone());
    }

    commands.spawn(
        floating_window_root("Crafting".into(), (
                UiCraftingRoot,
                Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                    parent.spawn((
                            UiRecipes,
                            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                                for recipe in recipes.iter() {
                                    let tmp = recipe.clone();
                                    parent.spawn((
                                            UiRecipe(tmp.clone()),
                                            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                                                parent.spawn((
                                                        UiRecipeIcon,
                                                ));
                                                parent.spawn((
                                                        UiRecipeText(tmp.clone()),
                                                ));
                                            })),
                                    ));
                                }
                            })),
                    ));
                    parent.spawn((
                            UiActiveRecipe(None),
                    ));
                }))
        )
    ));
}

fn on_recipe_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    recipe_query: Query<&UiRecipe>,
    mut acive_recipe_query: Query<&mut UiActiveRecipe>,
) {
    if let Ok(recipe) = recipe_query.get(trigger.entity)
    && let Ok(mut active_recipe) = acive_recipe_query.single_mut() {
        active_recipe.0 = Some(recipe.0.clone());
    }
}

fn sync_active_recipe(
    recipe_book: Res<RecipeBook>,
    mut commands: Commands,
    mut changed_active_recipe: Query<(Entity, &mut UiActiveRecipe), Changed<UiActiveRecipe>>,
    player_inventory_query: Query<Option<&Inventory>, With<Player>>,
    tag_query: Query<&CraftTag>,
) {
    if let Ok((active_entity, mut active_recipe)) = changed_active_recipe.single_mut()
    && let Ok(inventory) = player_inventory_query.single() {

        commands.entity(active_entity).despawn_children();

        if active_recipe.0.is_some() {
            let text = active_recipe.0.clone().unwrap();
            let recipe = recipe_book.0.get(&text).unwrap();
            let inputs = recipe.inputs.clone();
            let desc = recipe.description.clone();


            let mut tally: HashMap<String, u32> = HashMap::new();
            if inventory.is_some() {
                tally = tally_tags(inventory.unwrap(), &tag_query);
                println!("{:?}", tally);
            }

            let craftable = recipe_is_craftable(recipe, &tally);

            let icon = commands.spawn((
                    Node {
                        width: Val::Px(100.),
                        height: Val::Px(100.),
                        ..default()
                    },
                    BackgroundColor::from(YELLOW),
            )).id();

            let name = commands.spawn((
                    Node {
                        ..default()
                    },
                    BackgroundColor::from(GREEN),
                    Text(text.clone()),
            )).id();

            let description = commands.spawn((
                    Node {
                        ..default()
                    },
                    BackgroundColor::from(GREEN),
                    Text(desc),
            )).id();

            let input_node = commands.spawn((
                    Node {
                        ..default()
                    },
                    BackgroundColor::from(DARK_KHAKI),
                    Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                        for (input, ammount) in inputs {
                            let mut tmp = tally.get(&input).unwrap_or(&0);

                            let text = format!(" {tmp} / {ammount}");

                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    BackgroundColor::from(BLUE),
                                    Text(input),
                            ));
                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    BackgroundColor::from(BLUE),
                                    Text(text),
                            ));
                        }
                    })),
            )).id();

            let progress = commands.spawn((
                    UiCraftingProgressBar,
                    ProgressBar {
                        value: 0.,
                        output: Val::Percent(100.),
                    },
                    ProgressTimer(Timer::from_seconds(60., TimerMode::Repeating)),
            )).id();

            let mut background = BackgroundColor::from(GRAY);
            if craftable {
                background = BackgroundColor::from(BLUE);
            }

            let craft_button = commands.spawn((
                    UiCraftButton(craftable),
                    background,
            )).id();

            commands.entity(active_entity).add_children(&[icon, name, description, input_node, progress, craft_button]);
        }
    }
}

fn on_craft_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    crafting_station_query: Query<Entity, With<CraftingStation>>,
    mut active_recipe: Query<&mut UiActiveRecipe>,
) {
    if let Ok(crafting_station) = crafting_station_query.single()
    && let Ok(mut active) = active_recipe.single_mut() {
        commands.entity(crafting_station).trigger(|entity| CraftEvent { entity, id: "fungacide".into()});
        let tmp = active.0.clone();
        active.0 = None;
        active.0 = tmp;
    }
}
