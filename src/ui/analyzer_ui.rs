use bevy::{app::Propagate, color::palettes::css::{DARK_KHAKI, DARK_RED, DARK_SLATE_GRAY, DARK_TURQUOISE, DARK_VIOLET, LIGHT_PINK, PURPLE, SADDLE_BROWN}, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{ActiveSample, AnalyzeSampleEvent, Analyzer, AnalyzerTimer, DisplayInventoryEvent, InvRef, Inventory, ItemDetails, Owner, SampleItem, UiInventory, UiInventoryItem, UiState, widgets::{floating_windows::floating_window_root, progress_bar::ProgressBar}};

#[derive(Component)]
pub struct ProgressTimer(pub Timer);

#[derive(EntityEvent)]
#[entity_event(propagate, auto_propagate)]
pub struct RefreshAnalyzerUi {
    pub entity: Entity,
}

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
#[component(on_add = on_ui_analyzer_root_add)]
pub struct UiAnalyzerRoot;

fn on_ui_analyzer_root_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity);
}

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        flex_direction: FlexDirection::Column,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    Text("AEIOU".into()),
    BackgroundColor::from(DARK_SLATE_GRAY),
)]
#[component(on_add = on_ui_analyzer_add)]
pub struct UiAnalyzer;

fn on_ui_analyzer_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity);
}

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        justify_content: JustifyContent::Center,
        ..default()
    },
    BackgroundColor::from(SADDLE_BROWN),
)]
#[component(on_add = on_ui_active_sample_add)]
pub struct UiActiveSample;

fn on_ui_active_sample_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity);
}

#[derive(Component, Reflect)]
#[require(
    Node {
        align_self: AlignSelf::Center,
        padding: UiRect::all(px(24)),
        ..default()
    },
    Text("ACTIVE".into()),
    BackgroundColor::from(DARK_RED),
)]
#[component(on_add = on_ui_active_sample_icon_add)]
pub struct UiActiveSampleIcon;

fn on_ui_active_sample_icon_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(start_sample_analysis)
        .observe(refresh_analyzer_ui_observer);
}

#[allow(clippy::complexity)]
fn start_sample_analysis(
    trigger: On<Pointer<DragDrop>>,
    mut commands: Commands,
    owner_query: Query<&Owner>,
    invref_query: Query<&InvRef>,
    childof_query: Query<&ChildOf>,
    sample_query: Query<&SampleItem>,
    analyzer_query: Query<Entity, With<Analyzer>>,
    active_sample_query: Query<Entity, With<ActiveSample>>,
) {
    if trigger.event().button == PointerButton::Secondary {
        return;
    }
    if let Ok(invref) = invref_query.get(trigger.entity)
    && let Ok(item) = owner_query.get(trigger.dropped)
    && let Ok(childof) = childof_query.get(trigger.dropped)
    && let Ok(analyzer) = analyzer_query.single() {
        for active in active_sample_query.iter() {
            commands.entity(active).remove::<ActiveSample>();
        }
        commands.entity(analyzer).trigger(|entity| AnalyzeSampleEvent(item.item_owner));
        //commands.entity(trigger.entity).trigger(|entity| RefreshAnalyzerUi { entity });
    }
}

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        justify_content: JustifyContent::Start,
        ..default()
    },
    BackgroundColor::from(PURPLE),
)]
pub struct UiAnalysisProgress;

#[derive(Component, Reflect)]
#[require(
    Node {
        padding: UiRect::all(px(24)),
        flex_grow: 1.,
        justify_content: JustifyContent::Center,
        row_gap: px(20),
        column_gap: px(20),
        ..default()
    },
    BackgroundColor::from(LIGHT_PINK),
)]
pub struct UiAnalysisInput;

#[derive(Component, Reflect)]
#[require(
    Node {
        align_self: AlignSelf::Center,
        padding: UiRect::all(px(24)),
        ..default()
    },
    Text("X".into()),
    BackgroundColor::from(DARK_KHAKI),
)]
#[component(on_add = on_ui_analysis_input_cancel_add)]
pub struct UiAnalysisInputCancel;

fn on_ui_analysis_input_cancel_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(on_analysis_cancel_observer);
}

fn on_analysis_cancel_observer(
    _trigger: On<Pointer<Click>>,
    mut commands: Commands,
    active_sample_query: Query<Entity, With<ActiveSample>>,
) {
    if let Ok(active_sample) = active_sample_query.single() {
        commands.entity(active_sample).remove::<ActiveSample>();
    }
}

#[derive(Component, Reflect)]
#[require(
    Node {
        align_self: AlignSelf::Center,
        padding: UiRect::all(px(24)),
        ..default()
    },
    Text("|| / >".into()),
    BackgroundColor::from(DARK_TURQUOISE),
)]
#[component(on_add = on_ui_analysis_input_pause_add)]
pub struct UiAnalysisInputPause;

fn on_ui_analysis_input_pause_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(on_analysis_pause_observer);
}

fn on_analysis_pause_observer(
    _trigger: On<Pointer<Click>>,
    mut commands: Commands,
    analyzer_query: Query<Entity, With<Analyzer>>,
) {
    if let Ok(analyzer) = analyzer_query.single() {
        commands.entity(analyzer).trigger(|entity| AnalyzeSampleEvent(entity));
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
    ProgressBar {
        value: 0.,
        output: Val::Percent(100.),
    },
)]
pub struct UiAnalyzerProgressBar;

fn update_ui_analyzer_progress(
    mut ui_query: Query<&mut ProgressBar, With<UiAnalysisProgress>>,
    craft_query: Query<&AnalyzerTimer>,
) {
    if let Ok(mut ui) = ui_query.single_mut()
    && let Ok(timer) = craft_query.single() {
        ui.value = timer.0.fraction();
    }
}

pub struct AnalyzerUiPlugin;
impl Plugin for AnalyzerUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(Update, (
                   //update_progress_bar,
                   sync_analyzer_ui,
                   update_ui_analyzer_progress,
           ));
    }
}

pub fn update_progress_bar(
    time: Res<Time>,
    mut progress_bar_query: Query<(&mut ProgressBar, &mut ProgressTimer)>,
) {
    if let Ok((mut progress_bar, mut progress_timer)) = progress_bar_query.single_mut() { //println!("{:?}", progress_bar.value);
        progress_timer.0.tick(time.delta());
        progress_bar.value = progress_timer.0.fraction();
    }
}

#[allow(clippy::complexity)]
pub fn display_analyzer_ui(
    trigger: On<DisplayInventoryEvent>,
    mut commands: Commands,
    name_query: Query<&Name>,
    item_query: Query<&ItemDetails>,
    inventory: Query<&Inventory>,
    menu_state: Res<State<UiState>>,
    mut menu_state_setter: ResMut<NextState<UiState>>,
    active_sample_query: Query<Entity, With<ActiveSample>>,
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

    let mut active_sample = String::from("ACTIVESAMPLE");

    if let Ok(active_sample_entity) = active_sample_query.single()
    && let Ok(active_sample_name) = name_query.get(active_sample_entity) {
        println!("ITEM: {:?}", active_sample_entity);
        active_sample = active_sample_name.into();
    }

    let inv_ref = trigger.entity.clone();
    let inv_ref2 = trigger.entity.clone();

    commands.spawn((
            floating_window_root("Analyzer".into(), (
                    UiAnalyzerRoot,
                    Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                        parent.spawn((
                                UiInventory,
                                Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                                    for (item, entity, inv) in item_vec {
                                        parent.spawn((
                                                UiInventoryItem,
                                                Text(item.name),
                                                Owner { item_owner: entity, inv_owner: inv },
                                                Propagate( Owner { item_owner: entity, inv_owner: inv }),
                                        ));
                                    }
                                })),
                                InvRef(inv_ref.clone()),
                        ));
                        parent.spawn((
                                UiAnalyzer,
                                Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                                    parent.spawn((
                                            UiActiveSample,
                                            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                                                parent.spawn((
                                                        UiActiveSampleIcon,
                                                        Text(active_sample),
                                                        InvRef(inv_ref2),
                                                ));
                                            })),
                                    ));
                                    parent.spawn((
                                            UiAnalysisProgress,
                                            Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                                                parent.spawn((
                                                        UiAnalyzerProgressBar,
                                                        //ProgressBar {
                                                        //    value: 0.,
                                                        //    output: Val::Percent(100.),
                                                        //},
                                                        //ProgressTimer(Timer::from_seconds(60., TimerMode::Repeating)),
                                                ));
                                            })),
                                    ));
                                    parent.spawn((
                                            UiAnalysisInput,
                                            Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                                                parent.spawn(UiAnalysisInputCancel);
                                                parent.spawn(UiAnalysisInputPause);
                                            })),
                                    ));
                                })),
                        ));
                    })),
            )),
    ));
}

fn refresh_analyzer_ui_observer(
    _trigger: On<RefreshAnalyzerUi>,
    mut commands: Commands,
    name_query: Query<&Name>,
    active_sample_node_query: Query<Entity, With<UiActiveSampleIcon>>,
    active_sample_query: Query<Entity, With<ActiveSample>>,
) {
    let mut active_sample = String::from("ACTIVESAMPLE");

    if let Ok(active_sample_entity) = active_sample_query.single()
    && let Ok(active_sample_name) = name_query.get(active_sample_entity) {
        println!("ITEM: {:?}", active_sample_entity);
        active_sample = active_sample_name.into();
    }

    if let Ok(entity) = active_sample_node_query.single() {
        commands.entity(entity)
            .remove::<Text>()
            .insert(Text(active_sample));
    }
}

fn sync_analyzer_ui(
    mut removed: RemovedComponents<ActiveSample>,
    mut commands: Commands,
    name_query: Query<&Name>,
    active_sample_node_query: Query<Entity, With<UiActiveSampleIcon>>,
    active_sample_query: Query<Entity, Added<ActiveSample>>,
    analyzer_timer_query: Query<&AnalyzerTimer>,
) {
    let mut active_sample = String::from("ACTIVESAMPLE");

    removed.read().for_each(|entity| {
        println!("ACTIVE SAMPLE REMOVED");
        if let Ok(entity) = active_sample_node_query.single() {
            commands.entity(entity)
                .remove::<Text>()
                .insert(Text(active_sample.clone()));
        }
    });

    if let Ok(active_sample_entity) = active_sample_query.single()
    && let Ok(active_sample_name) = name_query.get(active_sample_entity) {
        println!("ITEM: {:?}", active_sample_entity);
        active_sample = active_sample_name.into();
        if let Ok(entity) = active_sample_node_query.single() {
            println!("ACTIVE SAMPLE ADDED");
            commands.entity(entity)
                .remove::<Text>()
                .insert(Text(active_sample));
        }
    }
}
