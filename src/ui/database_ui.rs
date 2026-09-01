use bevy::{color::palettes::css::{BLACK, BLUE, DARK_CYAN}, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{DiscoveredItems, GameState, ItemDatabase, container_interaction_observer, widgets::floating_windows::floating_window_root};

#[derive(Resource)]
struct DatabaseActiveEntry(pub String);

#[derive(Component, Reflect, Clone, PartialEq, Eq, Hash, Debug)]
#[reflect(Component)]
#[require(
    crate::Interactable,
    Name::new("Database Station"),
)]
#[component(on_add = on_ui_database_station_add)]
pub struct DatabaseStation;

fn on_ui_database_station_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(container_interaction_observer)
        .observe(display_database_ui);
}

#[derive(Component)]
#[require(
    Node {
        flex_grow: 1.,
        ..default()
    }
)]
pub struct UiDatabaseRoot;

#[derive(Component)]
#[require(
    Node {
        ..default()
    },
    BackgroundColor::from(BLUE),
)]
pub struct UiDatabaseEntryItems;

#[derive(Component)]
#[require(
    Node {
        height: Val::Percent(15.),
        ..default()
    },
    BackgroundColor::from(DARK_CYAN),
)]
#[component(on_add = on_ui_database_entry_add)]
pub struct UiDatabaseEntry(pub String);

fn on_ui_database_entry_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(on_ui_database_entry_click);
}

fn on_ui_database_entry_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    query: Query<&UiDatabaseEntry>,
) {
    if let Ok(entry) = query.get(trigger.entity) {
        commands.insert_resource(DatabaseActiveEntry(entry.0.clone()));
    }
}

#[derive(Component)]
#[require(
    Node {
        align_self: AlignSelf::Center,
        align_items: AlignItems::Center,
        flex_grow: 1.,
        overflow: Overflow { x: OverflowAxis::Clip, y: OverflowAxis::Scroll },
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        flex_wrap: FlexWrap::Wrap,
        height: Val::Percent(100.),
        ..default()
    },
    BackgroundColor::from(BLACK),
)]
pub struct UiDatabaseActiveEntry;

fn update_ui_database_active_entry(
    mut commands: Commands,
    active_entry: Option<Res<DatabaseActiveEntry>>,
    mut previous_active: Local<String>,
    item_database: Res<ItemDatabase>,
    active_entry_node_query: Query<Entity, With<UiDatabaseActiveEntry>>,
) {
    if active_entry.is_none() { return; }
    let active_entry = active_entry.unwrap();
    if *previous_active == active_entry.0 { return; }
    *previous_active = active_entry.0.clone();
    if let Some(item_details) = item_database.0.get(&active_entry.0)
    && let Ok(active_entry_node) = active_entry_node_query.single() {
        let icon = commands.spawn((
                UiDatabaseActiveEntryIcon,
            )).id();
        let title = commands.spawn((
                UiDatabaseActiveEntryTitle,
                Text(item_details.name.clone()),
            )).id();
        let desc = commands.spawn((
                UiDatabaseActiveEntryDesc,
                Text(item_details.description.clone())
            )).id();

        commands.entity(active_entry_node).add_child(icon);
        commands.entity(active_entry_node).add_child(title);
        commands.entity(active_entry_node).add_child(desc);
    }
}

#[derive(Component)]
#[require(
    Node {
        ..default()
    },
    BackgroundColor::from(BLACK),
)]
pub struct UiDatabaseActiveEntryIcon;

#[derive(Component)]
#[require(
    Node {
        ..default()
    },
    BackgroundColor::from(BLACK),
)]
pub struct UiDatabaseActiveEntryTitle;

#[derive(Component)]
#[require(
    Node {
        ..default()
    },
    BackgroundColor::from(BLACK),
)]
pub struct UiDatabaseActiveEntryDesc;

pub struct DatabaseUiPlugin;
impl Plugin for DatabaseUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(Update, update_ui_database_active_entry.run_if(in_state(GameState::Gameplay)));
    }
}

fn display_database_ui(
    _trigger: On<crate::DisplayInventoryEvent>,
    mut commands: Commands,
    discovered_items: Res<DiscoveredItems>,
    item_database: Res<ItemDatabase>,
) {
    let mut analyzed_vec = vec![];
    for item in discovered_items.0.iter() {
        if let Some(item) = item_database.0.get(item) {
            analyzed_vec.push(item.id.clone());
        }
    }

    commands.spawn((
            floating_window_root("Database".into(), (
                UiDatabaseRoot,
                Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                    parent.spawn((
                            UiDatabaseEntryItems,
                            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                                for item in analyzed_vec {
                                    parent.spawn((
                                            UiDatabaseEntry(item.clone()),
                                            Text(item),
                                    ));
                                }
                            })),
                    ));
                    parent.spawn((
                            UiDatabaseActiveEntry,
                    ));
                })),
            )),
    ));
}
