use bevy::{color::palettes::css::{BLACK, BLUE, DARK_CYAN}, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{GameState, widgets::floating_windows::floating_computer_window_root};

#[derive(Resource)]
pub struct DatabaseActiveEntry(pub String);
impl FromWorld for DatabaseActiveEntry {
    fn from_world(_world: &mut World) -> Self {
        Self("".to_string())
    }
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
        flex_direction: FlexDirection::Column,
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
    } else {
        error!("on_ui_database_entry_click: Failed to query UiDatabaseEntry");
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
#[component(on_add = on_ui_database_active_entry_add)]
pub struct UiDatabaseActiveEntry;

fn on_ui_database_active_entry_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut res = world.resource_mut::<DatabaseActiveEntry>();
    res.0 = "".to_string();

    world.commands()
        .entity(context.entity);
}

pub struct DatabaseAppPlugin;
impl Plugin for DatabaseAppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_ui_database_active_entry.run_if(in_state(GameState::Gameplay)));
    }
}

fn update_ui_database_active_entry(
    mut commands: Commands,
    active_entry: Option<Res<DatabaseActiveEntry>>,
    mut previous_active: Local<String>,
    item_database: Res<crate::ItemDatabase>,
    active_entry_node_query: Query<Entity, With<UiDatabaseActiveEntry>>,
) {
    if active_entry.is_none() { return; }
    let active_entry = active_entry.unwrap();
    if *previous_active == active_entry.0 { return; }
    *previous_active = active_entry.0.clone();

    let Some(item_details) = item_database.0.get(&active_entry.0) else {
        error!("update_ui_database_active_entry: Failed to get ItemDefinition for {}", active_entry.0);
        return;
    };
    let Ok(active_entry_node) = active_entry_node_query.single() else {
        error!("update_ui_database_active_entry: Failed to query Entity for UiDatabaseActiveEntry");
        return;
    };

    commands.entity(active_entry_node).despawn_children();
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

#[derive(Component)]
#[require(
    Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(10.),
        height: Val::Percent(15.),
        align_items: AlignItems::Center,
        border_radius: BorderRadius::all(Val::Px(10.)),
        left: Val::Px(20.),
        top: Val::Px(200.),
        flex_direction: FlexDirection::Column,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor(BLUE.into()),
    super::IconClickTimer(Timer::from_seconds(1.0, TimerMode::Once)),
)]
#[component(on_add = on_computer_database_icon_add)]
pub struct ComputerDatabaseIcon;

fn on_computer_database_icon_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let icon: Handle<Image> = world.resource::<AssetServer>().load("icons/geometrica/save-block.png");

    let icon_node = world.commands().spawn((
        Node {
            width: Val::Auto,
            height: Val::Percent(75.),
            ..default()
        },
        ImageNode::new(icon),
    )).id();

    let text_node = world.commands().spawn((
        Node {
            width: Val::Auto,
            height: Val::Percent(25.),
            ..default()
        },
        Text::new("DATABASE"),
    )).id();

    world.commands()
        .entity(context.entity)
        .observe(super::icon_drag_observer)
        .observe(super::icon_over)
        .observe(super::icon_out)
        .observe(database_icon_double_click_observer)
        .add_child(icon_node)
        .add_child(text_node);
}

fn database_icon_double_click_observer(
    trigger: On<Pointer<Click>>,
    mut timer_query: Query<&mut super::IconClickTimer>,
    mut commands: Commands,
    computer_ui_query: Query<Entity, With<crate::ComputerUiNode>>,
    discovered_items: Res<crate::DiscoveredItems>,
    item_database: Res<crate::ItemDatabase>,
) {
    trace!("OBSERVER: database_icon_double_click_observer");

    let Ok(mut timer) = timer_query.get_mut(trigger.entity) else {
        error!("cctv_icon_double_click_observer: Failed to query IconClickTimer for {}", trigger.entity);
        return;
    };
    let Ok(computer_ui) = computer_ui_query.single() else {
        error!("database_icon_double_click_observer: Failed to query Entity for ComputerUiNode");
        return;
    };

    let mut analyzed_vec = vec![];
    for item in discovered_items.0.iter() {
        if let Some(item) = item_database.0.get(item) {
            analyzed_vec.push(item.id.clone());
        }
    }

    if timer.0.is_finished() {
        timer.0.reset();
    } else {
        let window = commands.spawn((
                floating_computer_window_root("Database".into(), (
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
        )).id();

        commands.entity(computer_ui).add_child(window);
    }
}
