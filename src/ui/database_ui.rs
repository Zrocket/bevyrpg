use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Analyzed, ItemDetails, widgets::floating_windows::floating_window_root};

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
        .observe(display_database_ui);
}

#[derive(Component)]
#[require(
    Node {
        ..default()
    }
)]
pub struct UiDatabaseRoot;

#[derive(Component)]
#[require(
    Node {
        ..default()
    }
)]
pub struct UiDatabaseEntryItems;

#[derive(Component)]
#[require(
    Node {
        ..default()
    }
)]
#[component(on_add = on_ui_database_entry_add)]
pub struct UiDatabaseEntry;

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
) {
}

#[derive(Component)]
#[require(
    Node {
        ..default()
    }
)]
pub struct UiDatabaseActiveEntry;

pub struct DatabaseUiPlugin;
impl Plugin for DatabaseUiPlugin {
    fn build(&self, app: &mut App) {
       app;
    }
}

fn display_database_ui(
    _trigger: On<crate::DisplayInventoryEvent>,
    mut commands: Commands,
    analyzed_query: Query<Entity, With<Analyzed>>,
    item_query: Query<&ItemDetails>,
) {
    let mut analyzed_vec = vec![];
    for analyzed in analyzed_query.iter() {
        if let Ok(item) = item_query.get(analyzed) {
            analyzed_vec.push(item.clone());
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
                                            UiDatabaseEntry,
                                            Text(item.name),
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
