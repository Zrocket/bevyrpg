use std::collections::HashSet;

use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use serde::Deserialize;

use crate::{ItemId, add_to_inventory_observer, analyzer_ui::display_analyzer_ui, container_interaction_observer};

#[derive(Component, Reflect, Clone, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub struct AnalyzerTimer(pub Timer);

#[derive(Resource, Deserialize, Clone, Debug)]
pub struct AnalysisResults(pub Vec<String>);

#[derive(Component, Reflect, Clone, Default, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub struct Analyzed;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct DiscoveredItems(pub HashSet<String>);

fn update_analyzer_timer(
    mut timer_query: Query<(&mut AnalyzerTimer, &ActiveSample)>,
    mut discovered_items: ResMut<DiscoveredItems>,
    name_query: Query<&Name>,
    item_id_query: Query<&ItemId>,
    time: Res<Time>,
) {
    if let Ok((mut timer, active_sample)) = timer_query.single_mut() {
        if timer.0.is_finished()
        //&& let Ok(value) = name_query.get(active_sample.0)
        && let Ok(item_id) = item_id_query.get(active_sample.0) {
            discovered_items.0.insert(item_id.0.clone());
        } else {
            timer.0.tick(time.delta());
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    crate::Interactable,
    Name::new("Analyzer"),
)]
#[component(on_add = on_analyzer_add)]
pub struct Analyzer {
    pub sample: Option<Entity>,
    active: bool,
    progress: f32,
}

fn on_analyzer_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(add_to_inventory_observer::<Analyzer>)
        .observe(analyze_sample_event_observer)
        .observe(container_interaction_observer)
        .observe(display_analyzer_ui);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ActiveSample(pub Entity);

#[derive(EntityEvent)]
pub struct AnalyzeSampleCancel(pub Entity);

#[derive(EntityEvent)]
pub struct AnalyzeSamplePause(pub Entity);

#[derive(EntityEvent)]
pub struct AnalyzeSampleEvent{
    pub entity: Entity,
    pub sample: Entity,
}

pub struct AnalyzerPlugin;
impl Plugin for AnalyzerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(Update, update_analyzer_timer)
           .init_resource::<DiscoveredItems>();
    }
}

fn analyze_sample_event_observer(
    trigger: On<AnalyzeSampleEvent>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity).insert(AnalyzerTimer(Timer::from_seconds(60., TimerMode::Once)));
    commands.entity(trigger.entity).remove::<ActiveSample>();
    commands.entity(trigger.entity).insert(ActiveSample(trigger.sample));
}
